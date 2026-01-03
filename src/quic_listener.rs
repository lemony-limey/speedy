use tokio::net::{ToSocketAddrs, UdpSocket};
use socket2::SockAddr;
use crate::quic_stream::QuicStream;

#[derive(Debug)]
pub struct QuicListener
{
    socket: SockAddr,
    udp: UdpSocket,
}

impl QuicListener
{
    pub fn bind<A>(address: A) -> anyhow::Result<Self>
    where
        A: ToSocketAddrs
    {
        todo!()
    }

    pub fn accept(&self) -> anyhow::Result<(QuicStream, SockAddr)>
    {
        todo!()
    }
}

pub struct Incoming
{

}
