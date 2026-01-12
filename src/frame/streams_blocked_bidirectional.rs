use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Copy, Debug)]
pub struct StreamsBlockedBidirectional
{
    // Bidirectional: 0x16
    frame_type:      VariableLengthInteger,
    maximum_streams: VariableLengthInteger,
}

impl StreamsBlockedBidirectional
{
    pub fn new(maximum_streams: VariableLengthInteger) -> Self
    {
        Self {
            frame_type: VariableLengthInteger::from(FrameType::StreamsBlockedBidirectional),
            maximum_streams,
        }
    }
}
