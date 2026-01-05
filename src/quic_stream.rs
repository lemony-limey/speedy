use socket2::SockAddr;
use std::io::IoSlice;
use std::net::Shutdown;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::{ToSocketAddrs, UdpSocket};
use crate::variable_length_integer::VariableLengthInteger;

pub enum StreamType
{
    ClientInitiatedBidirectional  = 0x00,
    ServerInitiatedBidirectional  = 0x01,
    ClientInitiatedUnidirectional = 0x02,
    ServerInitiatedUnidirectional = 0x03,
}

#[derive(Debug)]
pub struct QuicStream
{
    udp_socket:  UdpSocket,
    stream_id:   VariableLengthInteger,
    local_addr:  SockAddr,
    remote_addr: SockAddr,
}

impl QuicStream
{
    pub fn connect<A>(address: A) -> anyhow::Result<Self>
    where
        A: ToSocketAddrs
    {
        todo!()
    }

    pub fn shutdown(&self, how: Shutdown) -> anyhow::Result<()>
    {
        todo!()
    }

    /// TODO: Perhaps make this an immutable reference
    pub fn remote_addr(&self) -> anyhow::Result<SockAddr>
    {
        Ok(self.remote_addr.clone())
    }

    /// TODO: Perhaps make this an immutable reference
    pub fn local_addr(&self) -> anyhow::Result<SockAddr>
    {
        Ok(self.local_addr.clone())
    }
}

impl AsyncRead for &QuicStream
{
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>>
    {
        self.udp_socket.poll_recv(cx, buf)
    }
}

impl AsyncRead for QuicStream
{
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>>
    {
        self.udp_socket.poll_recv(cx, buf)
    }
}

impl AsyncWrite for &QuicStream
{
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<std::io::Result<usize>>
    {
        self.udp_socket.poll_send(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>>
    {
        todo!()
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>>
    {
        todo!()
    }

    fn poll_write_vectored(self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>]) -> Poll<std::io::Result<usize>>
    {
        todo!()
    }

    fn is_write_vectored(&self) -> bool
    {
        todo!()
    }
}

impl AsyncWrite for QuicStream
{
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<std::io::Result<usize>>
    {
        self.udp_socket.poll_send(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>>
    {
        todo!()
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>>
    {
        todo!()
    }

    fn poll_write_vectored(self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>]) -> Poll<std::io::Result<usize>>
    {
        todo!()
    }

    fn is_write_vectored(&self) -> bool
    {
        todo!()
    }
}
