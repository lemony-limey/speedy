use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Copy, Debug)]
pub struct RetireConnectionID
{
    frame_type: VariableLengthInteger,
    sequence_number: VariableLengthInteger,
}

impl RetireConnectionID
{
    pub fn new(sequence_number: VariableLengthInteger) -> Self
    {
        Self {
            frame_type: VariableLengthInteger::from(FrameType::RetireConnectionID),
            sequence_number,
        }
    }
}
