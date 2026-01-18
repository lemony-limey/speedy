use nom::error::ErrorKind;
use nom::IResult;
use crate::parser::BitInput;
use crate::parser::bits::{take_bits_u16, take_bits_u32, take_bits_u64, take_bits_u8};
use crate::variable_length_integer::VariableLengthInteger;


/// Parse a variable length integer from the byte stream.
pub(crate) fn parse_variable_length_integer(input: BitInput) -> IResult<BitInput, VariableLengthInteger>
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


#[cfg(test)]
mod tests
{
    use crate::parser::variable_length_integer::parse_variable_length_integer;
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
