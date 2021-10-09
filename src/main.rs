mod lib;
use lib::nameserver;
use lib::udp::udp_service;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:3000".to_string());
    let mut ns = nameserver::bind(&addr.parse().unwrap()).await;
    ns.serve(udp_service::UdpService::new()).await?;
    Ok(())
}
