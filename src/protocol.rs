use libp2p::Multiaddr;
use serde::{Deserialize, Serialize};

pub const AGENT_VERSION: &'static str = "peer/0.0.1";
pub const PROTOCOL: &'static str = "/foo/1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreetRequest {
    pub message: String,
    pub address: Multiaddr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreetResponse {
    pub message: String,
    pub address: Multiaddr,
}
