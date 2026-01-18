use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Copy, Debug)]
pub struct StreamsBlocked
{
    // Bidirectional:  0x16
    // Unidirectional: 0x17
    frame_type:      FrameType,
    maximum_streams: VariableLengthInteger,
}

impl StreamsBlocked
{
    pub fn new(
        frame_type: FrameType,
        maximum_streams: VariableLengthInteger
    ) -> Self
    {
        Self {
            frame_type,
            maximum_streams,
        }
    }

    pub fn new_bidirectional(maximum_streams: VariableLengthInteger) -> Self
    {
        Self {
            frame_type: FrameType::StreamsBlockedBidirectional,
            maximum_streams,
        }
    }

    pub fn new_unidirectional(maximum_streams: VariableLengthInteger) -> Self
    {
        Self {
            frame_type: FrameType::StreamsBlockedUnidirectional,
            maximum_streams,
        }
    }
}
