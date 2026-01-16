use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Debug)]
pub struct MaxData
{
    frame_type:   FrameType,
    maximum_data: VariableLengthInteger,
}

impl MaxData
{
    pub fn new(
        maximum_data: VariableLengthInteger,
    ) -> Self
    {
        Self {
            frame_type: FrameType::MaxData,
            maximum_data,
        }
    }
}
