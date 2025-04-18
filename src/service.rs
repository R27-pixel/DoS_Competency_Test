use crate::protocol::{GreetRequest, GreetResponse};
use std::task::{Context, Poll};
use tower::Service;

#[derive(Clone)]
pub struct GreetService;

impl Service<GreetRequest> for GreetService {
    type Response = GreetResponse;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: GreetRequest) -> Self::Future {
        println!("Handling REQ via Tower: {}", req.message);
        let resp = GreetResponse {
            message: "Hello Back!! R27".to_string(),
            address: req.address.clone(),
        };
        Box::pin(async move { Ok(resp) })
    }
}
