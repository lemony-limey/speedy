use bytes::Bytes;
use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

/// Crypto frames CANNOT be sent in 0-RTT packets.
#[derive(Clone, Debug)]
pub struct Crypto
{
    frame_type: VariableLengthInteger,
    offset:     VariableLengthInteger,
    length:     VariableLengthInteger,
    data:       Bytes,
}

impl Crypto
{
    pub fn new(
        offset: VariableLengthInteger,
        length: VariableLengthInteger,
        data:   Bytes,
    ) -> Self
    {
        Self {
            frame_type: VariableLengthInteger::from(FrameType::Crypto),
            offset,
            length,
            data,
        }
    }
}
