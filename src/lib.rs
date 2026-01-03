use std::io;
use anyhow::anyhow;

mod frames;
mod packets;
mod parser;
mod quic_listener;
mod quic_socket;
mod quic_stream;

/// The largest value that can be stored in an instance of VariableLengthInteger::EightBit.
/// Anything larger than this value must be stored with a u16.
const VARIABLE_U8_MAX: u8 = 63;  // (2^6) - 1
/// The largest value that can be stored in an instance of VariableLengthInteger::SixteenBit
/// Anything larger than this value must be stored with a u32.
const VARIABLE_U16_MAX: u16 = 16_383;  // (2^14) - 1
/// The largest value that can be stored in an instance of VariableLengthInteger::ThirtyTwoBit
/// Anything larger than this value must be stored with a u64.
const VARIABLE_U32_MAX: u32 = 1_073_741_823;  // (2^30) - 1
/// The largest value that can be stored in an instance of VariableLengthInteger::SixtyFourBit
/// Anything larger than this value must be stored with a u128, but this is not supported
/// by the QUIC variable-length encoding.
const VARIABLE_U64_MAX: u64 = 4_611_686_018_427_387_903;  // (2^62) - 1

/// An implementation of the type described in RFC 9000, Section 16.
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum VariableLengthInteger
{
    EightBit(u8),      // 00
    SixteenBit(u16),   // 01
    ThirtyTwoBit(u32), // 10
    SixtyFourBit(u64), // 11
}

impl VariableLengthInteger
{
    /// This should only be performed if it can be guaranteed that the value will fit into
    /// 6 bits.
    /// If this is not the case, information will be lost when the bit mask is applied.
    /// TODO: Specify bit mask.
    unsafe fn new_u8_unchecked(value: u8) -> Self
    {
        // TODO: Add bit mask
        todo!();
        VariableLengthInteger::EightBit(value)
    }

    /// This should only be performed if it can be guaranteed that the value will fit into
    /// 6 bits.
    /// If this is not the case, information will be lost when the bit mask is applied.
    /// TODO: Specify bit mask.
    unsafe fn new_u16_unchecked(value: u16) -> Self
    {
        // TODO: Add bit mask
        todo!();
        VariableLengthInteger::SixteenBit(value)
    }

    /// This should only be performed if it can be guaranteed that the value will fit into
    /// 6 bits.
    /// If this is not the case, information will be lost when the bit mask is applied.
    /// TODO: Specify bit mask.
    unsafe fn new_u32_unchecked(value: u32) -> Self
    {
        // TODO: Add bit mask
        todo!();
        VariableLengthInteger::ThirtyTwoBit(value)
    }

    /// This should only be performed if it can be guaranteed that the value will fit into
    /// 6 bits.
    /// If this is not the case, information will be lost when the bit mask is applied.
    /// TODO: Specify bit mask.
    unsafe fn new_u64_unchecked(value: u64) -> Self
    {
        // TODO: Add bit mask
        todo!();
        VariableLengthInteger::SixtyFourBit(value)
    }
}

impl TryFrom<u8> for VariableLengthInteger
{
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error>
    {
        if value <= VARIABLE_U8_MAX
        {
            // No bit mask necessary here
            Ok(VariableLengthInteger::EightBit(value))
        }
        else  // Requires a u32
        {
            // // SAFETY: This call is guaranteed to succeed, because an 8-bit value
            // // will always fit into 14 bits.
            // unsafe { Ok(Self::new_u16_unchecked(value as u16)) }

            Self::try_from(value as u16)
        }
    }
}

impl TryFrom<u16> for VariableLengthInteger
{
    type Error = anyhow::Error;

    fn try_from(mut value: u16) -> Result<Self, Self::Error>
    {
        if value <= VARIABLE_U8_MAX as u16
        {
            Self::try_from(value as u8)
        }
        else if value <= VARIABLE_U16_MAX
        {
            // The 2 most significant bits are set to "01"
            value |= 0b0100_0000_0000_0000;
            Ok(VariableLengthInteger::SixteenBit(value))
        }
        else  // Requires a u32
        {
            Self::try_from(value as u32)
        }
    }
}

impl TryFrom<u32> for VariableLengthInteger
{
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error>
    {
        if value <= VARIABLE_U8_MAX as u32
        {
            Self::try_from(value as u8)
        }
        else if value <= VARIABLE_U16_MAX as u32
        {
            Self::try_from(value as u16)
        }
        else if value <= VARIABLE_U32_MAX
        {
            // TODO: Apply bit mask
            // The 2 most significant bits are set to "10"
            Ok(VariableLengthInteger::ThirtyTwoBit(value))
        }
        else  // Requires a u64
        {
            Self::try_from(value as u64)
        }
    }
}

impl TryFrom<u64> for VariableLengthInteger
{
    type Error = anyhow::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error>
    {
        if value <= VARIABLE_U8_MAX as u64
        {
            Self::try_from(value as u8)
        }
        else if value <= VARIABLE_U16_MAX as u64
        {
            Self::try_from(value as u16)
        }
        else if value <= VARIABLE_U32_MAX as u64
        {
            Self::try_from(value as u32)
        }
        else if value <= VARIABLE_U64_MAX
        {
            // TODO: Apply bit mask
            // The 2 most significant bits are set to "10"
            Ok(VariableLengthInteger::SixtyFourBit(value))
        }
        else  // Too large to be stored as a QUIC variable-length integer.
        {
            Err(anyhow!(
                "Cannot store {} as a variable-length integer, as it it larger than \
                the maximum accepted value: {}",
                value,
                VARIABLE_U64_MAX,
            ))
        }
    }
}
