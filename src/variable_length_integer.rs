use anyhow::anyhow;
use crate::variable_length_integer::encoded_u8::{VariableLengthEncodedU8, VARIABLE_LENGTH_U8_MAX};
use crate::variable_length_integer::encoded_u16::{VariableLengthEncodedU16, VARIABLE_LENGTH_U16_MAX};
use crate::variable_length_integer::encoded_u32::{VariableLengthEncodedU32, VARIABLE_LENGTH_U32_MAX};
use crate::variable_length_integer::encoded_u64::{VariableLengthEncodedU64, VARIABLE_LENGTH_U64_MAX};

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
            Ok(VariableLengthInteger::EightBit(
                VariableLengthEncodedU8::try_new_from_decoded_value(value)?
            ))
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

    fn try_from(value: u16) -> Result<Self, Self::Error>
    {
        if value <= VARIABLE_LENGTH_U8_MAX as u16
        {
            Self::try_from(value as u8)
        }
        else if value <= VARIABLE_LENGTH_U16_MAX
        {
            Ok(VariableLengthInteger::SixteenBit(
                VariableLengthEncodedU16::try_new_from_decoded_value(value)?
            ))
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

    fn try_from(value: u32) -> Result<Self, Self::Error>
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
            Ok(VariableLengthInteger::ThirtyTwoBit(
                VariableLengthEncodedU32::try_new_from_decoded_value(value)?
            ))
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

    fn try_from(value: u64) -> Result<Self, Self::Error>
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
            Ok(VariableLengthInteger::SixtyFourBit(
                VariableLengthEncodedU64::try_new_from_decoded_value(value)?
            ))
        }
        else  // Too large to be stored as a QUIC variable-length integer.
        {
            Err(anyhow!(
                "Cannot store {} as a variable-length integer, as it it larger than \
                    the maximum accepted value: {}",
                value,
                VARIABLE_LENGTH_U64_MAX,
            ))
        }
    }
}


/// Defines the interface that all variable length integer types must conform to.
/// This makes it easier to work with these types, since it is known that regardless of the size
/// of the current value, the operations that can be performed are the same.
/// TODO: Split into multiple smaller traits? (i.e., VariableLengthEncode and VariableLengthDecode?)
///     This would certainly make it easier to tell what TryFrom should be attempting to convert
///     from.
pub trait VariableLengthEncodeDecode<T>
{
    fn try_new_from_decoded_value(decoded_value: T) -> anyhow::Result<Self> where Self: Sized;
    fn new_from_encoded_value(encoded_value: T) -> Self;
    fn decoded_value(&self) -> T;
    fn encoded_value(&self) -> T;
}

/// From for this can take a decoded value and encode it.
pub trait VariableLengthEncode<T>
{
    /// Take a decoded value and encode it.
    fn try_new_from_decoded_value(decoded_value: T) -> anyhow::Result<Self> where Self: Sized;
    /// Retrieve the encoded value.
    fn encoded_value(&self) -> T;
}

/// From for this can take an encoded value and decode it.
pub trait VariableLengthDecode<T>
{
    /// We can now take the encoded value and decode it.
    fn new_from_encoded_value(encoded_value: T) -> Self;
    /// Retrieve the decoded value.
    fn decoded_value(&self) -> T;
}


#[cfg(test)]
mod tests
{
    use anyhow::anyhow;
    use crate::variable_length_integer::{VariableLengthEncodeDecode, VariableLengthInteger};
    use crate::variable_length_integer::encoded_u8::VARIABLE_LENGTH_U8_MAX;
    use crate::variable_length_integer::encoded_u16::{VARIABLE_LENGTH_U16_BIT_SETTING_MASK, VARIABLE_LENGTH_U16_MAX};
    use crate::variable_length_integer::encoded_u32::{VARIABLE_LENGTH_U32_BIT_SETTING_MASK, VARIABLE_LENGTH_U32_MAX};
    use crate::variable_length_integer::encoded_u64::{VARIABLE_LENGTH_U64_BIT_SETTING_MASK, VARIABLE_LENGTH_U64_MAX};

    #[test]
    fn test_valid_u8_conversion() -> anyhow::Result<()>
    {
        let zero = 0u8;
        let ten = 10u8;
        let six_bit_max = VARIABLE_LENGTH_U8_MAX;
        let greater_than_six_bit_max = VARIABLE_LENGTH_U8_MAX + 1;

        let VariableLengthInteger::EightBit(from_zero) = VariableLengthInteger::try_from(zero)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::EightBit(from_ten) = VariableLengthInteger::try_from(ten)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::EightBit(from_six_bit_max) = VariableLengthInteger::try_from(six_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::SixteenBit(from_gt_six_bit_max) = VariableLengthInteger::try_from(greater_than_six_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };

        assert_eq!(zero, from_zero.decoded_value());
        assert_eq!(zero, from_zero.encoded_value());

        assert_eq!(ten, from_ten.decoded_value());
        assert_eq!(ten, from_ten.encoded_value());

        assert_eq!(six_bit_max, from_six_bit_max.decoded_value());
        assert_eq!(six_bit_max, from_six_bit_max.encoded_value());

        // The value greater than the six bit max should be converted to a u16.
        assert_eq!(greater_than_six_bit_max as u16, from_gt_six_bit_max.decoded_value());
        assert_eq!(
            greater_than_six_bit_max as u16 | VARIABLE_LENGTH_U16_BIT_SETTING_MASK,
            from_gt_six_bit_max.encoded_value(),
        );

        Ok(())
    }


    #[test]
    fn test_valid_u16_conversion_decoded() -> anyhow::Result<()>
    {
        let zero = 0u16;
        let ten = 10u16;
        let greater_than_six_bit_max = (VARIABLE_LENGTH_U8_MAX + 1) as u16;
        let fourteen_bit_max = VARIABLE_LENGTH_U16_MAX;
        let greater_than_fourteen_bit_max = VARIABLE_LENGTH_U16_MAX + 1;

        // Even though zero and ten are u16 values, they will be converted to
        // VariableLengthEncodedU8s, as this will save space.
        let VariableLengthInteger::EightBit(from_zero) = VariableLengthInteger::try_from(zero)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::EightBit(from_ten) = VariableLengthInteger::try_from(ten)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::SixteenBit(from_gt_six_bit_max) = VariableLengthInteger::try_from(greater_than_six_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::SixteenBit(from_fourteen_bit_max) = VariableLengthInteger::try_from(fourteen_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::ThirtyTwoBit(from_gt_fourteen_bit_max) = VariableLengthInteger::try_from(greater_than_fourteen_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };

        // 0 and 10 will both become VariableLengthEncodedU8s because using 14 bits to store
        // these values is excessive.
        assert_eq!(zero as u8, from_zero.decoded_value());
        assert_eq!(zero as u8, from_zero.encoded_value());
        assert_eq!(ten as u8, from_ten.decoded_value());
        assert_eq!(ten as u8, from_ten.encoded_value());

        // 64 will need to become a VariableLengthEncodedU16 as 6 bits is not sufficient to
        // represent this value, so 14 bits must be used instead.
        assert_eq!(
            greater_than_six_bit_max,
            from_gt_six_bit_max.decoded_value(),
        );
        assert_eq!(
            greater_than_six_bit_max | VARIABLE_LENGTH_U16_BIT_SETTING_MASK,
            from_gt_six_bit_max.encoded_value(),
        );

        assert_eq!(
            fourteen_bit_max,
            from_fourteen_bit_max.decoded_value(),
        );
        assert_eq!(
            fourteen_bit_max | VARIABLE_LENGTH_U16_BIT_SETTING_MASK,
            from_fourteen_bit_max.encoded_value(),
        );

        // The value greater than the fourteen bit max should be converted to a u32.
        assert_eq!(
            greater_than_fourteen_bit_max as u32,
            from_gt_fourteen_bit_max.decoded_value(),
        );
        assert_eq!(
            greater_than_fourteen_bit_max as u32 | VARIABLE_LENGTH_U32_BIT_SETTING_MASK,
            from_gt_fourteen_bit_max.encoded_value(),
        );

        Ok(())
    }


    // #[test]
    // fn test_valid_u16_conversion_encoded() -> anyhow::Result<()>
    // {
    //     let greater_than_six_bit_max_encoded = (VARIABLE_LENGTH_U8_MAX + 1) as u16 | VARIABLE_LENGTH_U16_BIT_SETTING_MASK;
    //     let fourteen_bit_max_encoded = VARIABLE_LENGTH_U16_MAX | VARIABLE_LENGTH_U16_BIT_SETTING_MASK;
    //
    //     let VariableLengthInteger::SixteenBit(from_gt_six_bit_max) = VariableLengthEncodeDecode::new_from_encoded_value(greater_than_six_bit_max_encoded)? else {
    //         return Err(anyhow!("oh no!"))
    //     };
    //     let VariableLengthInteger::SixteenBit(from_fourteen_bit_max) = VariableLengthEncodeDecode::new_from_encoded_value(fourteen_bit_max_encoded)? else {
    //         return Err(anyhow!("oh no!"))
    //     };
    //
    //     // 64 will need to become a VariableLengthEncodedU16 as 6 bits is not sufficient to
    //     // represent this value, so 14 bits must be used instead.
    //     assert_eq!(
    //         greater_than_six_bit_max | VARIABLE_LENGTH_U16_BIT_SETTING_MASK,
    //         from_gt_six_bit_max.encoded_value(),
    //     );
    //     assert_eq!(
    //         greater_than_six_bit_max_encoded,
    //         from_gt_six_bit_max.decoded_value(),
    //     );
    //
    //     assert_eq!(
    //         fourteen_bit_max,
    //         from_fourteen_bit_max.decoded_value(),
    //     );
    //     assert_eq!(
    //         fourteen_bit_max | VARIABLE_LENGTH_U16_BIT_SETTING_MASK,
    //         from_fourteen_bit_max.encoded_value(),
    //     );
    //
    //     // The value greater than the fourteen bit max should be converted to a u32.
    //     assert_eq!(
    //         greater_than_fourteen_bit_max as u32,
    //         from_gt_fourteen_bit_max.decoded_value(),
    //     );
    //     assert_eq!(
    //         greater_than_fourteen_bit_max as u32 | VARIABLE_LENGTH_U32_BIT_SETTING_MASK,
    //         from_gt_fourteen_bit_max.encoded_value(),
    //     );
    //
    //     Ok(())
    // }


    #[test]
    fn test_valid_u32_conversion_decoded() -> anyhow::Result<()>
    {
        let zero = 0u32;
        let ten = 10u32;
        let greater_than_six_bit_max = (VARIABLE_LENGTH_U8_MAX + 1) as u32;
        let fourteen_bit_max = VARIABLE_LENGTH_U16_MAX as u32;
        let greater_than_fourteen_bit_max = (VARIABLE_LENGTH_U16_MAX + 1) as u32;
        let thirty_bit_max = VARIABLE_LENGTH_U32_MAX;
        let greater_than_thirty_bit_max = VARIABLE_LENGTH_U32_MAX + 1;

        // Even though zero and ten are u16 values, they will be converted to
        // VariableLengthEncodedU8s, as this will save space.
        let VariableLengthInteger::EightBit(from_zero) = VariableLengthInteger::try_from(zero)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::EightBit(from_ten) = VariableLengthInteger::try_from(ten)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::SixteenBit(from_gt_six_bit_max) = VariableLengthInteger::try_from(greater_than_six_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::SixteenBit(from_fourteen_bit_max) = VariableLengthInteger::try_from(fourteen_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::ThirtyTwoBit(from_gt_fourteen_bit_max) = VariableLengthInteger::try_from(greater_than_fourteen_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::ThirtyTwoBit(from_thirty_bit_max) = VariableLengthInteger::try_from(thirty_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::SixtyFourBit(from_gt_thirty_bit_max) = VariableLengthInteger::try_from(greater_than_thirty_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };

        // 0 and 10 will both become VariableLengthEncodedU8s because using 14 bits to store
        // these values is excessive.
        assert_eq!(zero as u8, from_zero.decoded_value());
        assert_eq!(zero as u8, from_zero.encoded_value());
        assert_eq!(ten as u8, from_ten.decoded_value());
        assert_eq!(ten as u8, from_ten.encoded_value());

        // 64 will need to become a VariableLengthEncodedU16 as 6 bits is not sufficient to
        // represent this value, so 14 bits must be used instead.
        assert_eq!(
            greater_than_six_bit_max as u16,
            from_gt_six_bit_max.decoded_value(),
        );
        assert_eq!(
            greater_than_six_bit_max as u16 | VARIABLE_LENGTH_U16_BIT_SETTING_MASK,
            from_gt_six_bit_max.encoded_value(),
        );

        assert_eq!(
            fourteen_bit_max as u16,
            from_fourteen_bit_max.decoded_value(),
        );
        assert_eq!(
            fourteen_bit_max as u16 | VARIABLE_LENGTH_U16_BIT_SETTING_MASK,
            from_fourteen_bit_max.encoded_value(),
        );

        // 64 and 1_073_741_823 should be converted to u32s.
        assert_eq!(
            greater_than_fourteen_bit_max,
            from_gt_fourteen_bit_max.decoded_value(),
        );
        assert_eq!(
            greater_than_fourteen_bit_max | VARIABLE_LENGTH_U32_BIT_SETTING_MASK,
            from_gt_fourteen_bit_max.encoded_value(),
        );

        assert_eq!(
            thirty_bit_max,
            from_thirty_bit_max.decoded_value(),
        );
        assert_eq!(
            thirty_bit_max | VARIABLE_LENGTH_U32_BIT_SETTING_MASK,
            from_thirty_bit_max.encoded_value(),
        );

        // 1_073_741_824 must be converted to a u64.
        assert_eq!(
            greater_than_thirty_bit_max as u64,
            from_gt_thirty_bit_max.decoded_value(),
        );
        assert_eq!(
            greater_than_thirty_bit_max as u64 | VARIABLE_LENGTH_U64_BIT_SETTING_MASK,
            from_gt_thirty_bit_max.encoded_value(),
        );

        Ok(())
    }


    #[test]
    fn test_valid_u64_conversion_decoded() -> anyhow::Result<()>
    {
        let zero = 0u64;
        let ten = 10u64;
        let greater_than_six_bit_max = (VARIABLE_LENGTH_U8_MAX + 1) as u64;
        let fourteen_bit_max = VARIABLE_LENGTH_U16_MAX as u64;
        let greater_than_fourteen_bit_max = (VARIABLE_LENGTH_U16_MAX + 1) as u64;
        let thirty_bit_max = VARIABLE_LENGTH_U32_MAX as u64;
        let greater_than_thirty_bit_max = (VARIABLE_LENGTH_U32_MAX + 1) as u64;
        let sixty_two_bit_max = VARIABLE_LENGTH_U64_MAX;
        let greater_than_sixty_two_bit_max = VARIABLE_LENGTH_U64_MAX + 1;

        // Even though zero and ten are u16 values, they will be converted to
        // VariableLengthEncodedU8s, as this will save space.
        let VariableLengthInteger::EightBit(from_zero) = VariableLengthInteger::try_from(zero)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::EightBit(from_ten) = VariableLengthInteger::try_from(ten)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::SixteenBit(from_gt_six_bit_max) = VariableLengthInteger::try_from(greater_than_six_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::SixteenBit(from_fourteen_bit_max) = VariableLengthInteger::try_from(fourteen_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::ThirtyTwoBit(from_gt_fourteen_bit_max) = VariableLengthInteger::try_from(greater_than_fourteen_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::ThirtyTwoBit(from_thirty_bit_max) = VariableLengthInteger::try_from(thirty_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::SixtyFourBit(from_gt_thirty_bit_max) = VariableLengthInteger::try_from(greater_than_thirty_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };
        let VariableLengthInteger::SixtyFourBit(from_sixty_two_bit_max) = VariableLengthInteger::try_from(sixty_two_bit_max)? else {
            return Err(anyhow!("oh no!"))
        };
        // Attempting to convert a number that doesn't fit within 62 bits into a
        // VariableLengthEncodedU64 should return an error.
        let Err(_) = VariableLengthInteger::try_from(greater_than_sixty_two_bit_max) else {
            return Err(anyhow!("oh no!"))
        };

        // 0 and 10 will both become VariableLengthEncodedU8s because using 14 bits to store
        // these values is excessive.
        assert_eq!(zero as u8, from_zero.decoded_value());
        assert_eq!(zero as u8, from_zero.encoded_value());
        assert_eq!(ten as u8, from_ten.decoded_value());
        assert_eq!(ten as u8, from_ten.encoded_value());

        // 64 will need to become a VariableLengthEncodedU16 as 6 bits is not sufficient to
        // represent this value, so 14 bits must be used instead.
        assert_eq!(
            greater_than_six_bit_max as u16,
            from_gt_six_bit_max.decoded_value(),
        );
        assert_eq!(
            greater_than_six_bit_max as u16 | VARIABLE_LENGTH_U16_BIT_SETTING_MASK,
            from_gt_six_bit_max.encoded_value(),
        );

        assert_eq!(
            fourteen_bit_max as u16,
            from_fourteen_bit_max.decoded_value(),
        );
        assert_eq!(
            fourteen_bit_max as u16 | VARIABLE_LENGTH_U16_BIT_SETTING_MASK,
            from_fourteen_bit_max.encoded_value(),
        );

        // 64 and 1_073_741_823 should be converted to u32s.
        assert_eq!(
            greater_than_fourteen_bit_max as u32,
            from_gt_fourteen_bit_max.decoded_value(),
        );
        assert_eq!(
            greater_than_fourteen_bit_max as u32 | VARIABLE_LENGTH_U32_BIT_SETTING_MASK,
            from_gt_fourteen_bit_max.encoded_value(),
        );

        assert_eq!(
            thirty_bit_max as u32,
            from_thirty_bit_max.decoded_value(),
        );
        assert_eq!(
            thirty_bit_max as u32 | VARIABLE_LENGTH_U32_BIT_SETTING_MASK,
            from_thirty_bit_max.encoded_value(),
        );

        // 1_073_741_824 must be converted to a u64. It is the smallest value that a
        // VariableLengthEncodedU64 should contain, as any smaller value should fit into
        // fewer bits.
        assert_eq!(
            greater_than_thirty_bit_max,
            from_gt_thirty_bit_max.decoded_value(),
        );
        assert_eq!(
            greater_than_thirty_bit_max | VARIABLE_LENGTH_U64_BIT_SETTING_MASK,
            from_gt_thirty_bit_max.encoded_value(),
        );

        // The largest value that can fit into a VariableLengthEncodedU64.
        assert_eq!(
            sixty_two_bit_max,
            from_sixty_two_bit_max.decoded_value(),
        );
        assert_eq!(
            sixty_two_bit_max | VARIABLE_LENGTH_U64_BIT_SETTING_MASK,
            from_sixty_two_bit_max.encoded_value(),
        );

        Ok(())
    }
}
