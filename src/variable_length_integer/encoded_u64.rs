use anyhow::anyhow;

/// The largest value that can be stored in an instance of VariableLengthInteger::SixtyFourBit
/// Anything larger than this value must be stored with a u128, but this is not supported
/// by the QUIC variable-length encoding.
pub(crate) const VARIABLE_LENGTH_U64_MAX: u64 = 4_611_686_018_427_387_903;  // (2^62) - 1

/// The bit mask that must be applied to a u16 in order to indicate the length of the encoding.
const VARIABLE_LENGTH_U64_BIT_SETTING_MASK: u64 = 0xC000_0000_0000_0000;
const VARIABLE_LENGTH_U64_BIT_CLEARING_MASK: u64 = 0x3FFF_FFFF_FFFF_FFFF;


#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct VariableLengthEncodedU64
{
    decoded_value: u64,  // (No bit mask applied)
    encoded_value: u64,  // (Most significant two bits are masked to "11")
}

impl VariableLengthEncodedU64
{
    /// If a value is too large, this method will return Err.
    pub(crate) fn try_new_from_decoded_value(decoded_value: u64) -> anyhow::Result<Self>
    {
        // Ensure the value is within range
        if decoded_value > VARIABLE_LENGTH_U64_MAX
        {
            return Err(
                anyhow!(
                    "{} is too large to be encoded in 62 bits without loss of information (\
                        maximum is {} inclusive)",
                    decoded_value,
                    VARIABLE_LENGTH_U64_MAX,
                )
            )
        }

        let encoded_value = decoded_value | VARIABLE_LENGTH_U64_BIT_SETTING_MASK;

        Ok(
            Self
            {
                decoded_value,
                encoded_value,
            }
        )
    }

    pub(crate) fn new_from_encoded_value(encoded_value: u64) -> Self
    {
        let decoded_value = encoded_value & VARIABLE_LENGTH_U64_BIT_CLEARING_MASK;

        Self
        {
            decoded_value,
            encoded_value,
        }
    }

    pub fn decoded_value(&self) -> u64
    {
        self.decoded_value
    }

    pub fn encoded_value(&self) -> u64
    {
        self.encoded_value
    }
}
