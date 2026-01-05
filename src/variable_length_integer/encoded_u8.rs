use anyhow::anyhow;

/// The largest value that can be stored in an instance of VariableLengthInteger::EightBit.
/// Anything larger than this value must be stored with a u16.
pub(crate) const VARIABLE_LENGTH_U8_MAX: u8 = 63;  // (2^6) - 1


/// No bit mask is required for the value here, as the successful construction of an instance of
/// this type guarantees that the stored value can be stored in 6 bits.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct VariableLengthEncodedU8
{
    value: u8,
}

impl VariableLengthEncodedU8
{

    pub(crate) fn try_new_from_decoded_value(decoded_value: u8) -> anyhow::Result<Self>
    {
        // Ensure the value is within range
        if decoded_value > VARIABLE_LENGTH_U8_MAX
        {
            return Err(
                anyhow!(
                    "{} is too large to be encoded in 6 bits without loss of information (\
                        maximum is {} inclusive)",
                    decoded_value,
                    VARIABLE_LENGTH_U8_MAX,
                )
            )
        }

        Ok(
            Self { value: decoded_value }
        )
    }

    pub(crate) fn new_from_encoded_value(encoded_value: u8) -> Self
    {
        Self { value: encoded_value }
    }

    /// For the u8 variant, this method and the `encoded_value()` method will return the exact
    /// same value.
    pub fn decoded_value(&self) -> u8
    {
        self.value
    }

    pub fn encoded_value(&self) -> u8
    {
        self.value
    }
}
