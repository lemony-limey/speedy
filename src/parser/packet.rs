use nom::error::ErrorKind;
use nom::IResult;
use crate::frame::Frame;
use crate::packet::{Packet, PacketType};
use crate::parser::BitInput;
use crate::parser::bits::{take_bit_bool, take_bits_u32, take_bits_u8};
use crate::parser::header::{parse_long_header, parse_short_header};

/// Parse the header, then every frame in the packet.
pub fn parse_packet(input: BitInput, connection_id_length: Option<u8>) -> IResult<BitInput, Packet>
{
    // Determine the header type of the packet
    let (input, is_long_header_form) = take_bit_bool(input)?;

    // Short header is indicated by a 0. Only a 1-RTT packet uses a short header.
    // Long Header (header_form = 1)
    if is_long_header_form
    {
        // TODO: Parse next 39 bits and then bitshift left by 7 to ignore the first 7 bits for now.
        //  If the remaining result is 0, this is a version negotiation packet.

        // The second bit should be a 1
        let (input, is_long_header_form) = take_bit_bool(input)?;
        assert!(is_long_header_form);

        // The next two bits (3 & 4) indicate the packet type
        let (input, packet_type) = take_bits_u8(input, 2)?;

        // The first 4 bits of the input have been consumed at this point
        match PacketType::get_type_from_u8(packet_type)
        {
            PacketType::Initial   => parse_initial_packet(input),
            PacketType::ZeroRTT   => parse_0rtt_packet(input),
            PacketType::Handshake => parse_handshake_packet(input),
            PacketType::Retry     => parse_retry_packet(input),
            _                     => unimplemented!(),
        }
    }
    // Short Header (header_form = 0)
    else
    {
        // First bit of the input has been consumed
        let Some(connection_id_length) = connection_id_length else {
            return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::Fail }));
        };
        parse_1rtt_packet(input, connection_id_length)
    }
}


fn parse_1rtt_packet(input: BitInput, connection_id_length: u8) -> IResult<BitInput, Packet>
{
    // 1-RTT packets begin with a short header
    let (input, header) = parse_short_header(input, connection_id_length)?;

    let mut frames: Vec<Frame> = Vec::new();

    // Parse frames while the bytes stream is not empty
    // TODO: Potential source of bugs
    // TODO: Do we need to parse the frames, or can we simply just store the information contained
    //  inside?
    while input.0.is_empty()
    {

    }

    todo!()
}

fn parse_initial_packet(input: BitInput) -> IResult<BitInput, Packet>
{
    // Parse long header
    let (input, long_header) = parse_long_header(input)?;

    // Extract

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
    // Bits 5-8 are set to an arbitrary value
    const FOUR: usize = 4usize;
    let (input, _unused) = take_bits_u8(input, FOUR)?;

    const THIRTY_TWO: usize = 32usize;
    let (input, version) = take_bits_u32(input, 32usize)?;

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
