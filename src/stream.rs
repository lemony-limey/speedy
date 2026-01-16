use socket2::SockAddr;
use std::sync::Arc;
use crate::connection::Connection;
use crate::variable_length_integer::VariableLengthInteger;

pub enum StreamType
{
    ClientInitiatedBidirectional  = 0x00,
    ServerInitiatedBidirectional  = 0x01,
    ClientInitiatedUnidirectional = 0x02,
    ServerInitiatedUnidirectional = 0x03,
}

#[derive(Clone, Debug)]
pub struct RecvStream
{
    connection:  Arc<Connection>,
    stream_id:   VariableLengthInteger,
    local_addr:  SockAddr,
    remote_addr: SockAddr,
}

#[derive(Clone, Debug)]
pub struct SendStream
{
    connection:  Arc<Connection>,
    stream_id:   VariableLengthInteger,
    local_addr:  SockAddr,
    remote_addr: SockAddr,
}

#[derive(Clone, Copy, Debug)]
pub struct StreamID(VariableLengthInteger);
