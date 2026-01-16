use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

/// Contains the connection-level data limit at which blocking occurred.
#[derive(Clone, Copy, Debug)]
pub struct DataBlocked
{
    frame_type:   FrameType,
    maximum_data: VariableLengthInteger,
}

impl DataBlocked
{
    pub fn new(
        maximum_data: VariableLengthInteger,
    ) -> Self
    {
        Self {
            frame_type: FrameType::DataBlocked,
            maximum_data,
        }
    }
}
