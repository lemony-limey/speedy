use anyhow::anyhow;
use crate::variable_length_integer::encoded_u16::{VariableLengthEncodedU16, VARIABLE_LENGTH_U16_MAX};
use crate::variable_length_integer::encoded_u32::{VariableLengthEncodedU32, VARIABLE_LENGTH_U32_MAX};
use crate::variable_length_integer::encoded_u64::{VariableLengthEncodedU64, VARIABLE_LENGTH_U64_MAX};
use crate::variable_length_integer::encoded_u8::{VariableLengthEncodedU8, VARIABLE_LENGTH_U8_MAX};

pub mod encoded_u8;
pub mod encoded_u16;
pub mod encoded_u32;
pub mod encoded_u64;


/// An implementation of the variable-length unsigned integer type described in
/// RFC 9000, Section 16.
/// https://www.rfc-editor.org/rfc/rfc9000.html#name-variable-length-integer-enc
///
/// A variable-length encoding for non-negative integer values.
/// Integers can be encoded with 1, 2, 4 or 8 bytes (resulting in a
/// u8, u16, u32 and u64 respectively) depending on the value's magnitude; however,
/// since the first two bits indicate the length of the encoding, in reality we can
/// only use:
///     6 bits of the u8
///     14 bits of the u16
///     30 bits of the u32
///     62 bits of the u64
///
/// This means that in reality the maximum value we can encode with some type uX (where
/// u indicates an unsigned integer and X is the number of bits, 8, 16, 32 or 64 here)
/// is 2^(X - 2) - 1.
///
/// TODO: Swapping between the encoded and raw values is an absolute pain.
///     Separate structs for each?
///
/// We require the enum rather than a generic struct because of the edge case that sufficiently
/// large values may not fit into the data type with 2 fewer bits (i.e. 255 for u8 does not fit
/// into 6 bits).
/// This means that when encoding 255u8, we must return a VariableLengthEncodedU16, whereas if
/// a generic type was used, 255u8 could not be converted into VariableLengthInteger<u16>
/// without extra boilerplate code, for example.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum VariableLengthInteger
{
    EightBit(VariableLengthEncodedU8),      // 00 (i.e., no bitmask required)
    SixteenBit(VariableLengthEncodedU16),   // 01
    ThirtyTwoBit(VariableLengthEncodedU32), // 10
    SixtyFourBit(VariableLengthEncodedU64), // 11
}


impl TryFrom<u8> for VariableLengthInteger
{
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error>
    {
        if value <= VARIABLE_LENGTH_U8_MAX
        {
            // No bit mask necessary here, as first two bits must be "00" to be
            // within the allowed size anyway.
            Ok(VariableLengthInteger::EightBit(value))
        }
        else  // Requires a u32
        {
            // SAFETY: This call is guaranteed to succeed, because an 8-bit value
            // will always fit into 14 bits.
            Self::try_from(value as u16)
        }
    }
}

impl TryFrom<u16> for VariableLengthInteger
{
    type Error = anyhow::Error;

    fn try_from(mut value: u16) -> Result<Self, Self::Error>
    {
        if value <= VARIABLE_LENGTH_U8_MAX as u16
        {
            Self::try_from(value as u8)
        }
        else if value <= VARIABLE_LENGTH_U16_MAX
        {
            Ok(VariableLengthInteger::SixteenBit(value))
        }
        else  // Requires a u32
        {
            // SAFETY: This call is guaranteed to succeed, because a 16-bit value
            // will always fit into 30 bits.
            Self::try_from(value as u32)
        }
    }
}

impl TryFrom<u32> for VariableLengthInteger
{
    type Error = anyhow::Error;

    fn try_from(mut value: u32) -> Result<Self, Self::Error>
    {
        if value <= VARIABLE_LENGTH_U8_MAX as u32
        {
            Self::try_from(value as u8)
        }
        else if value <= VARIABLE_LENGTH_U16_MAX as u32
        {
            Self::try_from(value as u16)
        }
        else if value <= VARIABLE_LENGTH_U32_MAX
        {
            // The 2 most significant bits are set to "10"
            Ok(
                VariableLengthInteger::ThirtyTwoBit(
                    VariableLengthEncodedU32::try_new_from_decoded_value(value)?
                )
            )
        }
        else  // Requires a u64
        {
            // SAFETY: This call is guaranteed to succeed, because a 32-bit value
            // will always fit into 62 bits.
            Self::try_from(value as u64)
        }
    }
}

impl TryFrom<u64> for VariableLengthInteger
{
    type Error = anyhow::Error;

    fn try_from(mut value: u64) -> Result<Self, Self::Error>
    {
        if value <= VARIABLE_LENGTH_U8_MAX as u64
        {
            Self::try_from(value as u8)
        }
        else if value <= VARIABLE_LENGTH_U16_MAX as u64
        {
            Self::try_from(value as u16)
        }
        else if value <= VARIABLE_LENGTH_U32_MAX as u64
        {
            Self::try_from(value as u32)
        }
        else if value <= VARIABLE_LENGTH_U64_MAX
        {
            Ok(VariableLengthInteger::SixtyFourBit(value))
        }
        else  // Too large to be stored as a QUIC variable-length integer.
        {
            Err(
                anyhow!(
                    "Cannot store {} as a variable-length integer, as it it larger than \
                    the maximum accepted value: {}",
                    value,
                    VARIABLE_LENGTH_U64_MAX,
                )
            )
        }
    }
}


/// TODO: Just an idea at the moment.
pub trait VariableLengthEncoder
{
    type Value;
    type Error;

    fn encode_with_variable_length(value: Self::Value) -> Result<VariableLengthInteger, Self::Error>;
}


/// TODO: Just an idea at the moment.
pub trait VariableLengthCrap<T>
{
    type VariableLengthVariant;
    type Error;

    fn try_new_from_decoded_value(decoded_value: T) -> Result<Self::VariableLengthVariant, Self::Error>;
    fn new_from_encoded_value(encoded_value: T) -> Self;
    fn decoded_value(&self) -> T;
    fn encoded_value(&self) -> T;
}


#[cfg(test)]
mod tests
{
    use anyhow::anyhow;
    use crate::variable_length_integer::encoded_u8::VARIABLE_LENGTH_U8_MAX;
    use crate::variable_length_integer::VariableLengthInteger;

    #[test]
    fn test_valid_u8_conversion() -> anyhow::Result<()>
    {
        let zero = 0u8;
        let ten = 10u8;
        let six_bit_max = VARIABLE_LENGTH_U8_MAX;
        let gt_six_bit_max = VARIABLE_LENGTH_U8_MAX + 1;

        let VariableLengthInteger::EightBit(from_zero) = VariableLengthInteger::try_from(zero)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::EightBit(from_ten) = VariableLengthInteger::try_from(ten)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::EightBit(from_six_bit_max) = VariableLengthInteger::try_from(six_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::SixteenBit(from_gt_six_bit_max) = VariableLengthInteger::try_from(gt_six_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };

        assert_eq!(
            zero,
            from_zero.decoded_value(),
        );

        assert_eq!(
            ten,
            from_ten.decoded_value(),
        );

        assert_eq!(
            six_bit_max,
            from_six_bit_max.decoded_value(),
        );

        // The value greater than the six bit max should be converted to a u16.
        assert_eq!(
            gt_six_bit_max as u16,
            from_gt_six_bit_max.decoded_value(),
        );

        Ok(())
    }

    #[test]
    fn test_valid_u16_conversion()
    {

    }

    #[test]
    fn test_valid_u32_conversion()
    {

    }

    #[test]
    fn test_valid_u64_conversion()
    {

    }
}
