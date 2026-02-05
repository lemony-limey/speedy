use nom::error::ErrorKind;
use crate::packet::{LongHeader, ShortHeader};
use crate::parser::BitInput;
use nom::IResult;
use crate::parser::bits::{take_bit_bool, take_bits_u16, take_bits_u32, take_bits_u8};

pub(crate) fn parse_long_header(input: BitInput) -> IResult<BitInput, LongHeader>
{
    todo!()
}

/// Note that it is assumed that the first bit of the input has already been consumed at this point,
/// because if this was not the case the header type would not be known.
pub(crate) fn parse_short_header(input: BitInput, connection_id_length: u8) -> IResult<BitInput, ShortHeader>
{
    // Second bit should be fixed as a 1
    let (input, bit) = take_bit_bool(input)?;
    if !bit
    {
        return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::Fail }));
    }

    let (input, spin_bit) = take_bit_bool(input)?;

    // Reserved bits should be 0
    let (input, reserved) = take_bits_u8(input, 2usize)?;
    if reserved != 0
    {
        return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::Fail }));
    }

    let (input, key_phase) = take_bit_bool(input)?;
    let (input, packet_number_length) = take_bits_u8(input, 2usize)?;

    // TODO: Parse destination connection ID
    //  Issue: We need to know the length when receiving the packet.


    // TODO: Parse packet number
    // let (input, packet_number): (BitInput, u32);
    let (input, packet_number) = match packet_number_length
    {
        0 => {
            let (input, packet_number) = take_bits_u8(input, 8usize)?;
            (input, packet_number as u32)
        },
        1 => {
            let (input, packet_number) = take_bits_u16(input, 16usize)?;
            (input, packet_number as u32)
        },
        2 => {
            let (input, packet_number) = take_bits_u32(input, 24usize)?;
            (input, packet_number)
        },
        3 => {
            let (input, packet_number) = take_bits_u32(input, 32usize)?;
            (input, packet_number)
        },
        _ => {
            // This branch should be unreachable
            return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::Fail }));
        }
    };

    todo!();
    Ok((
        input,
        ShortHeader::new(
            spin_bit: val, key_phase: val, packet_number_length: val, dest_conn_id: val, packet_number: val
        )
    ))
}
