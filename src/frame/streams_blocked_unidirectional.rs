use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Copy, Debug)]
pub struct StreamsBlockedUnidirectional
{
    // Unidirectional: 0x17
    frame_type:      VariableLengthInteger,
    maximum_streams: VariableLengthInteger,
}

impl StreamsBlockedUnidirectional
{
    pub fn new(maximum_streams: VariableLengthInteger) -> Self
    {
        Self {
            frame_type: VariableLengthInteger::from(FrameType::StreamsBlockedUnidirectional),
            maximum_streams,
        }
    }
}
