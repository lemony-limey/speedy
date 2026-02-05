use crate::connection::Connection;
use crate::packet::Packet;
use crate::parser::packet::parse_packet;
use nom::IResult;

pub(crate) mod bits;
pub(crate) mod frame;
pub(crate) mod header;
pub(crate) mod packet;
pub(crate) mod variable_length_integer;


/// From: https://blog.adamchalmers.com/nom-bits/
pub(crate) type BitInput<'a> = (&'a [u8], usize);

/// TODO: Overall unit is the packet
pub(crate) fn parse_incoming<'a>(input: (&'a [u8], usize), connection: &Connection) -> IResult<BitInput<'a>, Packet>
{
    parse_packet(input, connection.connection_id_length)
}
