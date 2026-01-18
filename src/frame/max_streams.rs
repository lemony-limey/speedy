use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

/// MaxStreamsUnidirectional represents the count of the cumulative number of streams
/// ... that can be opened over the lifetime of the connection (RFC 9000, Section 19.11).
#[derive(Clone, Copy, Debug)]
pub struct MaxStreams
{
    // Bidirectional:  0x12
    // Unidirectional: 0x13
    frame_type:      FrameType,
    maximum_streams: VariableLengthInteger,
}

impl MaxStreams
{
    pub fn new(
        frame_type:      FrameType,
        maximum_streams: VariableLengthInteger,
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
            frame_type: FrameType::MaxStreamsBidirectional,
            maximum_streams,
        }
    }

    pub fn new_unidirectional(maximum_streams: VariableLengthInteger) -> Self
    {
        Self {
            frame_type: FrameType::MaxStreamsUnidirectional,
            maximum_streams,
        }
    }
}
