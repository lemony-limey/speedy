use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Copy, Debug)]
pub struct Ping
{
    frame_type: VariableLengthInteger,
}

impl Ping
{
    pub fn new() -> Self
    {
        Self { frame_type: VariableLengthInteger::from(FrameType::Ping) }
    }
}
