use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Copy, Debug)]
pub struct Padding
{
    frame_type: VariableLengthInteger,
}

impl Padding
{
    pub fn new() -> Self
    {
        Self { frame_type: VariableLengthInteger::from(FrameType::Padding) }
    }
}
