use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

pub struct HandshakeDone
{
    frame_type: VariableLengthInteger,
}

impl HandshakeDone
{
    pub fn new() -> Self
    {
        Self { frame_type: VariableLengthInteger::from(FrameType::HandshakeDone) }
    }

    pub fn frame_type(&self) -> &VariableLengthInteger
    {
        &self.frame_type
    }
}
