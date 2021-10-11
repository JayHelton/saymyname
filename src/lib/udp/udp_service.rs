use std::{pin::Pin, task::Poll};
use tower_service::Service;
use futures::Future;
use crate::lib::dns_query_parser::parser;

type BoxedError = Box<dyn std::error::Error + Sync + Send>;

pub struct UdpService;
impl UdpService {
    pub fn new() -> Self {
        UdpService {}
    }
}

impl Service<Vec<u8>> for UdpService {
    type Response = Vec<u8>;
    type Error = BoxedError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Vec<u8>) -> Self::Future {
        let msg = parser::Parser::parse(&req);
        let res = parser::Parser::compose(msg);
        Box::pin(async { Ok(res) })
    }
}
