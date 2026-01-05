use anyhow::anyhow;
use crate::variable_length_integer::VariableLengthEncodeDecode;

/// The largest value that can be stored in an instance of VariableLengthInteger::ThirtyTwoBit
/// Anything larger than this value must be stored with a u64.
pub(crate) const VARIABLE_LENGTH_U32_MAX: u32 = 1_073_741_823;  // (2^30) - 1

/// The bit mask that must be applied to a u16 in order to indicate the length of the encoding.
pub(crate) const VARIABLE_LENGTH_U32_BIT_SETTING_MASK: u32 = 0x8000_0000;
pub(crate) const VARIABLE_LENGTH_U32_BIT_CLEARING_MASK: u32 = 0x3FFF_FFFF;


#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct VariableLengthEncodedU32
{
    decoded_value: u32,  // (No bit mask applied)
    encoded_value: u32,  // (Most significant two bits are masked to "10")
}


impl VariableLengthEncodeDecode<u32> for VariableLengthEncodedU32
{
    /// If a value is too large, this method will return Err.
    fn try_new_from_decoded_value(decoded_value: u32) -> anyhow::Result<Self>
    {
        // Ensure the value is within range
        if decoded_value > VARIABLE_LENGTH_U32_MAX
        {
            return Err(anyhow!(
                "{} is too large to be encoded in 30 bits without loss of information (\
                    maximum is {} inclusive)",
                decoded_value,
                VARIABLE_LENGTH_U32_MAX,
            ));
        }

        let encoded_value = decoded_value | VARIABLE_LENGTH_U32_BIT_SETTING_MASK;

        Ok( Self { decoded_value, encoded_value } )
    }

    fn new_from_encoded_value(encoded_value: u32) -> Self
    {
        let decoded_value = encoded_value & VARIABLE_LENGTH_U32_BIT_CLEARING_MASK;

        Self { decoded_value, encoded_value }
    }

    fn decoded_value(&self) -> u32
    {
        self.decoded_value
    }

    fn encoded_value(&self) -> u32
    {
        self.encoded_value
    }
}
