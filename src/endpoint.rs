use std::sync::Arc;
use tokio::net::UdpSocket;

#[derive(Debug)]
pub struct Endpoint
{
    udp_socket: Arc<UdpSocket>,
}
