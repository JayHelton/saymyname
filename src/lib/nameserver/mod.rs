use std::{net::SocketAddr, sync::Arc};
use futures::Future;
use tokio::net::UdpSocket;
use tower_service::Service;

pub struct NameserverBuilder {
    socket: Arc<UdpSocket>,
}

impl NameserverBuilder {
    pub async fn serve<T>(&mut self, mut handler: T) -> Result<(), T::Error>
    where
        T: Service<Vec<u8>>,
        T::Error: From<std::io::Error> + Sync + Send, 
        T::Future: Future<Output = Result<Vec<u8>, T::Error>>,
    {
        loop {
            let mut buf = vec![0u8; 1024];
            match self.socket.recv_from(&mut buf).await {
                Ok((_, peer)) => {
                    let res = handler.call(buf.clone()).await?;
                    self.socket.send_to(&res, &peer).await?;

                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }
}

    pub async fn bind(addr: &SocketAddr) -> NameserverBuilder {
        let socket = UdpSocket::bind(addr).await.unwrap_or_else(|e| {
            panic!("error binding to {}: {}", addr, e);
        });
        NameserverBuilder { socket: Arc::new(socket) }
    }
