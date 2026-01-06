use std::sync::Arc;
use tokio::net::UdpSocket;

#[derive(Debug)]
pub struct QuicEndpoint
{
    udp_socket: Arc<UdpSocket>,
}
