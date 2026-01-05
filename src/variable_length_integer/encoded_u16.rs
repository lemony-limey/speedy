use anyhow::anyhow;

/// The largest value that can be stored in an instance of VariableLengthInteger::SixteenBit
/// Anything larger than this value must be stored with a u32.
pub(crate) const VARIABLE_LENGTH_U16_MAX: u16 = 16_383;  // (2^14) - 1

/// The bit mask that must be applied to a u16 in order to indicate the length of the encoding.
const VARIABLE_LENGTH_U16_BIT_SETTING_MASK: u16 = 0x4000;
const VARIABLE_LENGTH_U16_BIT_CLEARING_MASK: u16 = 0x3FFF;


#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct VariableLengthEncodedU16
{
    decoded_value: u16,  // (No bit mask applied)
    encoded_value: u16,  // (Most significant two bits are masked to "01")
}

impl VariableLengthEncodedU16
{
    pub(crate) fn try_new_from_decoded_value(decoded_value: u16) -> anyhow::Result<Self>
    {
        // Ensure the value is within range
        if decoded_value > VARIABLE_LENGTH_U16_MAX
        {
            return Err(
                anyhow!(
                    "{} is too large to be encoded in 14 bits without loss of information (\
                        maximum is {} inclusive)",
                    decoded_value,
                    VARIABLE_LENGTH_U16_MAX,
                )
            )
        }

        let encoded_value = decoded_value | VARIABLE_LENGTH_U16_BIT_SETTING_MASK;

        Ok(
            Self
            {
                decoded_value,
                encoded_value,
            }
        )
    }

    pub(crate) fn new_from_encoded_value(encoded_value: u16) -> Self
    {
        let decoded_value = encoded_value & VARIABLE_LENGTH_U16_BIT_CLEARING_MASK;

        Self
        {
            decoded_value,
            encoded_value,
        }
    }

    pub fn decoded_value(&self) -> u16
    {
        self.decoded_value
    }

    pub fn encoded_value(&self) -> u16
    {
        self.encoded_value
    }
}
