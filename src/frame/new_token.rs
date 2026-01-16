use bytes::Bytes;
use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Debug)]
pub struct NewToken
{
    frame_type:   FrameType,
    // Length of the token in bytes
    token_length: VariableLengthInteger,
    token:        Bytes,
}

impl NewToken
{
    pub fn new(
        token_length: VariableLengthInteger,
        token:        Bytes,
    ) -> Self
    {
        Self {
            frame_type: FrameType::NewToken,
            token_length,
            token,
        }
    }
}
