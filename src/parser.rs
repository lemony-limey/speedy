use nom::{IResult, Parser};
use nom::bits::bits;
use nom::combinator::peek;
use crate::packet::{Header, Packet, PacketType};
use crate::parser::bits::{take_bit_bool, take_bits_u8};

pub(crate) mod bits;
pub(crate) mod frames;
pub(crate) mod headers;
pub(crate) mod variable_length_integer;


/// From: https://blog.adamchalmers.com/nom-bits/
pub(crate) type BitInput<'a> = (&'a [u8], usize);


/// Parse the header, then every frame in the packet.
pub fn parse_packet(input: BitInput) -> IResult<BitInput, Packet>
{
    // Determine the header type of the packet
    let (input, is_long_header_form) = take_bit_bool(input)?;

    // Short header is indicated by a 0. Only a 1-RTT packet uses a short header.
    if is_long_header_form  // Long Header (header_form = 1)
    {
        // TODO: Check for version negotiation (bits [8, 39] should be 0).

        // The second bit should be a 1
        let (input, is_long_header_form) = take_bit_bool(input)?;
        assert!(is_long_header_form);

        // The next two bits (3 & 4) indicate the packet type
        let (input, packet_type) = take_bits_u8(input, 2)?;

        match PacketType::get_type_from_u8(packet_type)
        {
            PacketType::Initial   => parse_initial_packet(input),
            PacketType::ZeroRTT   => parse_0rtt_packet(input),
            PacketType::Handshake => parse_handshake_packet(input),
            PacketType::Retry     => parse_retry_packet(input),
            _                     => unimplemented!(),
        }
    }
    else  // Short Header (header_form = 0)
    {

        parse_1rtt_packet(input)
    }
}


fn parse_1rtt_packet(input: BitInput) -> IResult<BitInput, Packet>
{
    todo!()
}

fn parse_initial_packet(input: BitInput) -> IResult<BitInput, Packet>
{
    todo!()
}

fn parse_0rtt_packet(input: BitInput) -> IResult<BitInput, Packet>
{
    todo!()
}

fn parse_handshake_packet(input: BitInput) -> IResult<BitInput, Packet>
{
    todo!()
}

fn parse_retry_packet(input: BitInput) -> IResult<BitInput, Packet>
{
    todo!()
}

/// Version negotiation packets are awkward because it is only possibly to identify one if
/// the version field is set to 0, but the version field is at an offset of 8 bits.
///
/// Version negotiation packets will be ignored for now.
fn parse_version_negotiation_packet(input: BitInput) -> IResult<BitInput, Packet>
{
    todo!()
}
