use futures::StreamExt;
use libp2p::request_response::Message as RequestResponseMessage;
use libp2p::{
    Multiaddr, PeerId,
    core::ConnectedPoint,
    identify, identity, mdns,
    swarm::{
        NetworkBehaviour, SwarmEvent,
        dial_opts::{DialOpts, PeerCondition},
    },
};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
};
use tokio::{
    select,
    sync::mpsc,
    time::{Duration, interval},
};
use tower::{Service, ServiceExt};
use tracing_subscriber::filter::EnvFilter;

mod protocol;
mod service;

use protocol::{AGENT_VERSION, GreetRequest, GreetResponse, PROTOCOL};
use service::GreetService;

#[derive(NetworkBehaviour)]
struct Behaviour {
    identify: identify::Behaviour,
    mdns: mdns::tokio::Behaviour,
    request_response: libp2p::request_response::cbor::Behaviour<GreetRequest, GreetResponse>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("local peer id: {local_peer_id}");

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_quic()
        .with_behaviour(|key| Behaviour {
            identify: {
                let cfg = identify::Config::new(PROTOCOL.to_string(), key.public())
                    .with_push_listen_addr_updates(true)
                    .with_agent_version(AGENT_VERSION.to_string());
                identify::Behaviour::new(cfg)
            },
            mdns: mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())
                .unwrap(),
            request_response: {
                let cfg =
                    libp2p::request_response::Config::default().with_max_concurrent_streams(10);
                libp2p::request_response::cbor::Behaviour::<GreetRequest, GreetResponse>::new(
                    [(
                        libp2p::StreamProtocol::new(PROTOCOL),
                        libp2p::request_response::ProtocolSupport::Full,
                    )],
                    cfg,
                )
            },
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    if let Some(addr) = std::env::args().nth(1) {
        let addr = format!("/ip4/{}/udp/0/quic-v1", addr);
        swarm.listen_on(addr.parse()?)?;
    } else {
        swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    }

    let mut timer = interval(Duration::from_secs(5));
    let mut my_addr = Box::pin(Multiaddr::empty());
    let mut seen = Box::pin(BTreeSet::default());
    let mut in_peers = Box::pin(BTreeMap::<PeerId, Multiaddr>::default());
    let mut out_peers = Box::pin(BTreeMap::<PeerId, Multiaddr>::default());
    let (to_dial_send, mut to_dial_recv) = mpsc::unbounded_channel::<(PeerId, Multiaddr)>();
    let mut tower_service = GreetService;

    loop {
        select! {
            _ = timer.tick() => {
                let peers: Vec<_> = swarm.connected_peers().cloned().collect();
                for peer_id in peers {
                    if let Some(address) = out_peers.get(&peer_id) {
                        swarm.behaviour_mut().request_response.send_request(&peer_id, GreetRequest {
                            message: format!("Hello from Raunak{my_addr}!!"),
                            address: address.clone(),
                        });
                    }
                }
            }
            Some((peer_id, multiaddr)) = to_dial_recv.recv() => {
                let dial_opts = DialOpts::peer_id(peer_id)
                    .condition(PeerCondition::DisconnectedAndNotDialing)
                    .addresses(vec![multiaddr])
                    .build();
                let _ = swarm.dial(dial_opts);
            }
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(BehaviourEvent::Identify(identify::Event::Received { peer_id, info })) => {
                    if peer_id != local_peer_id && info.protocol_version != PROTOCOL {
                        println!("Disconnecting peer {peer_id} (wrong protocol)");
                        swarm.disconnect_peer_id(peer_id).ok();
                    }
                }
                SwarmEvent::Behaviour(BehaviourEvent::Mdns(event)) => match event {
                    mdns::Event::Discovered(list) => {
                        for (peer_id, multiaddr) in list {
                            if peer_id != local_peer_id && seen.insert(peer_id) {
                                to_dial_send.send((peer_id, multiaddr)).ok();
                            }
                        }
                    }
                    mdns::Event::Expired(list) => {
                        for (peer_id, _) in list {
                            seen.remove(&peer_id);
                            swarm.disconnect_peer_id(peer_id).ok();
                        }
                    }
                }
                SwarmEvent::Behaviour(BehaviourEvent::RequestResponse(libp2p::request_response::Event::Message { peer, message })) => match message {
                    RequestResponseMessage::Request { request, channel, .. } => {
                        let response = tower_service.ready().await?.call(request).await?;
                        swarm.behaviour_mut().request_response.send_response(channel, response).ok();
                    }
                    RequestResponseMessage::Response { response, .. } => {
                        println!("Response from {peer}: {}", response.message);
                        swarm.add_external_address(response.address);
                    }
                }
                SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => match endpoint {
                    ConnectedPoint::Dialer { address, .. } => {
                        out_peers.insert(peer_id, address);
                    }
                    ConnectedPoint::Listener { send_back_addr, .. } => {
                        in_peers.insert(peer_id, send_back_addr.clone());
                        if !out_peers.contains_key(&peer_id) {
                            to_dial_send.send((peer_id, send_back_addr)).ok();
                        }
                    }
                }
                SwarmEvent::ConnectionClosed { peer_id, .. } => {
                    in_peers.remove(&peer_id);
                    out_peers.remove(&peer_id);
                }
                SwarmEvent::ExternalAddrConfirmed { address } => {
                    *my_addr = address;
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    *my_addr = address;
                }
                _ => {}
            }
        }
    }
}
