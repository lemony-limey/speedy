use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Copy, Debug)]
pub struct PathResponse
{
    frame_type: VariableLengthInteger,
    data:       u64,
}

impl PathResponse
{
    pub fn new(data: u64) -> Self
    {
        Self {
            frame_type: VariableLengthInteger::from(FrameType::PathResponse),
            data,
        }
    }
}
