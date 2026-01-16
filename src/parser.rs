use crate::frame::{Frame, FrameType};
use crate::packet::Packet;
use crate::variable_length_integer::{VariableLengthDecode, VariableLengthInteger};
use nom::bits::bits;
use nom::bits::complete::take;
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::IResult;
use nom::Parser;

/// From: https://blog.adamchalmers.com/nom-bits/
type BitInput<'a> = (&'a [u8], usize);

/// Parse the header, then every frame in the packet.
pub fn parse_packet(input: &[u8]) -> anyhow::Result<Packet>
{
    // TODO: Parse header


    // TODO: Parse frame(s)


    todo!()
}

fn parse_frame(input: &[u8]) -> IResult<&[u8], Frame>
{
    // Frame type is always the first field, and is encoded as a variable length integer.
    let (input, VariableLengthInteger::EightBit(frame_type)) = bits(parse_variable_length_integer).parse(input)? else {
        // This should only fail if data runs out.
        return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::Fail }));
    };

    match FrameType::u8_to_frame_type(frame_type.decoded_value())
    {
        FrameType::Padding => Ok((input, Frame::Padding)),
        FrameType::Ping => Ok((input, Frame::Ping)),
        FrameType::Ack => unimplemented!(),
        FrameType::AckWithECN => unimplemented!(),
        FrameType::ResetStream => unimplemented!(),
        FrameType::StopSending => unimplemented!(),
        FrameType::Crypto => unimplemented!(),
        FrameType::NewToken => unimplemented!(),
        FrameType::StreamNoneSet => unimplemented!(),
        FrameType::StreamFin => unimplemented!(),
        FrameType::StreamLen => unimplemented!(),
        FrameType::StreamLenFin => unimplemented!(),
        FrameType::StreamOff => unimplemented!(),
        FrameType::StreamOffFin => unimplemented!(),
        FrameType::StreamOffLen => unimplemented!(),
        FrameType::StreamOffLenFin => unimplemented!(),
        FrameType::MaxData => unimplemented!(),
        FrameType::MaxStreamData => unimplemented!(),
        FrameType::MaxStreamsBidirectional => unimplemented!(),
        FrameType::MaxStreamsUnidirectional => unimplemented!(),
        FrameType::DataBlocked => unimplemented!(),
        FrameType::StreamDataBlocked => unimplemented!(),
        FrameType::StreamsBlockedBidirectional => unimplemented!(),
        FrameType::StreamsBlockedUnidirectional => unimplemented!(),
        FrameType::NewConnectionID => unimplemented!(),
        FrameType::RetireConnectionID => unimplemented!(),
        FrameType::PathChallenge => unimplemented!(),
        FrameType::PathResponse => unimplemented!(),
        FrameType::ConnectionCloseSuccessOrQuicError => unimplemented!(),
        FrameType::ConnectionCloseApplicationError => unimplemented!(),
        FrameType::HandshakeDone => Ok((input, Frame::HandshakeDone)),
    }
}

/// Parse a variable length integer from the byte stream.
fn parse_variable_length_integer(input: BitInput) -> IResult<BitInput, VariableLengthInteger>
{
    // Get the first two bits
    let (input, length) = take_bits_u8(input, 2)?;

    // Parse the rest of the integer depending on the specified length
    match length
    {
        // u8 - Parse the next 6 bits
        0b_00 => {
            let (input, value) = take_bits_u8(input, 6)?;
            Ok((input, VariableLengthInteger::from(value)))
        },
        // u16 - Parse the next 14 bits
        0b_01 => {
            let (input, value) = take_bits_u16(input, 14)?;
            Ok((input, VariableLengthInteger::from(value)))
        },
        // u32 - Parse the next 30 bits
        0b_10 => {
            let (input, value) = take_bits_u32(input, 30)?;
            Ok((input, VariableLengthInteger::from(value)))
        },
        // u64 - Parse the next 62 bits
        0b_11 => {
            let (input, value) = take_bits_u64(input, 62)?;

            // Conversion from 64-bit value to 62 bits is fallible, since 64 bits is the overall
            // maximum.
            let Ok(variable_length_integer) = VariableLengthInteger::try_from(value) else {
                return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::Fail }))
            };

            Ok((input, variable_length_integer))
        },
        _ => {
            // This should be unreachable.
            Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::Fail }))
        }
    }
}

/// Takes one bit from the input, returning true for 1 and false for 0.
/// This function is a modified version of one of the same name from:
/// https://blog.adamchalmers.com/nom-bits/
fn take_bit_bool(input: BitInput) -> IResult<BitInput, bool>
{
    map(
        take(1usize),
        |bit: u8| bit != 0
    ).parse(input)
}

fn take_bits_u8(input: BitInput, number_of_bits: usize) -> IResult<BitInput, u8>
{
    // If the number passed in is greater than 8, return Err.
    if number_of_bits > 8
    {
        return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::TooLarge }));
    }

    take(number_of_bits)(input)
}

fn take_bits_u16(input: BitInput, number_of_bits: usize) -> IResult<BitInput, u16>
{
    // If the number passed in is greater than 16, return Err.
    if number_of_bits > 16
    {
        return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::TooLarge }));
    }

    take(number_of_bits)(input)
}

fn take_bits_u32(input: BitInput, number_of_bits: usize) -> IResult<BitInput, u32>
{
    // If the number passed in is greater than 32, return Err.
    if number_of_bits > 32
    {
        return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::TooLarge }));
    }

    take(number_of_bits)(input)
}

fn take_bits_u64(input: BitInput, number_of_bits: usize) -> IResult<BitInput, u64>
{
    // If the number passed in is greater than 64, return Err.
    if number_of_bits > 64
    {
        return Err(nom::Err::Failure(nom::error::Error { input, code: ErrorKind::TooLarge }));
    }

    take(number_of_bits)(input)
}


#[cfg(test)]
mod tests
{
    use crate::parser::parse_variable_length_integer;
    use crate::variable_length_integer::{VariableLengthDecode, VariableLengthInteger};
    use nom::Finish;

    #[test]
    fn test_variable_length_integer_u8_conversion()
    {
        // This is BIG-ENDIAN.
        let value: [u8; 1] = [0b_00101010];

        let (_, variable_length_integer) = parse_variable_length_integer((&value, 0))
            .finish()
            .unwrap();

        let VariableLengthInteger::EightBit(variable_length_integer) = variable_length_integer else {
            panic!("This should not fail")
        };

        println!("{}", value[0]);
        println!("{:#?}", variable_length_integer);

        assert_eq!(value[0], variable_length_integer.decoded_value());
    }

    #[test]
    fn test_variable_length_integer_u16_conversion()
    {
        // This is BIG-ENDIAN.
        let value: [u8; 2] = [0b_01101010, 0b_10101010];

        let (_, variable_length_integer) = parse_variable_length_integer((&value, 0))
            .finish()
            .unwrap();

        let VariableLengthInteger::SixteenBit(variable_length_integer) = variable_length_integer else {
            panic!("This should not fail")
        };

        // Clear the first two bits so that the type isn't included. This is BIG-ENDIAN.
        let value: [u8; 2] = [0b_00101010, 0b_10101010];
        let value = u16::from_be_bytes(value);
        println!("{}", value);
        println!("{:#?}", variable_length_integer);

        assert_eq!(value, variable_length_integer.decoded_value());
    }

    #[test]
    fn test_variable_length_integer_u32_conversion()
    {
        // This is BIG-ENDIAN.
        let value: [u8; 4] = [0b_10110011, 0x_22, 0x_33, 0x_44];

        let (_, variable_length_integer) = parse_variable_length_integer((&value, 0))
            .finish()
            .unwrap();

        let VariableLengthInteger::ThirtyTwoBit(variable_length_integer) = variable_length_integer else {
            panic!("This should not fail")
        };

        // Clear the first two bits so that the type isn't included. This is BIG-ENDIAN.
        let value: [u8; 4] = [0b_00110011, 0x_22, 0x_33, 0x_44];
        let value = u32::from_be_bytes(value);
        println!("{}", value);
        println!("{:#?}", variable_length_integer);

        assert_eq!(value, variable_length_integer.decoded_value());
    }

    #[test]
    fn test_variable_length_integer_u64_conversion()
    {
        // This is BIG-ENDIAN.
        let value: [u8; 8] = [0b_11110011, 0x_22, 0x_33, 0x_44, 0x55, 0x66, 0x77, 0x88];

        let (_, variable_length_integer) = parse_variable_length_integer((&value, 0))
            .finish()
            .unwrap();

        let VariableLengthInteger::SixtyFourBit(variable_length_integer) = variable_length_integer else {
            panic!("This should not fail")
        };

        // Clear the first two bits so that the type isn't included. This is BIG-ENDIAN.
        let value: [u8; 8] = [0b_00110011, 0x_22, 0x_33, 0x_44, 0x55, 0x66, 0x77, 0x88];
        let value = u64::from_be_bytes(value);
        println!("{}", value);
        println!("{:#?}", variable_length_integer);

        assert_eq!(value, variable_length_integer.decoded_value());
    }
}
