use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

/// MaxStreamsUnidirectional represents the count of the cumulative number of [unidirectional]
/// streams ... that can be opened over the lifetime of the connection (RFC 9000, Section 19.11).
#[derive(Clone, Copy, Debug)]
pub struct MaxStreamsUnidirectional
{
    // Unidirectional: 0x13
    frame_type:      VariableLengthInteger,
    maximum_streams: VariableLengthInteger,
}

impl MaxStreamsUnidirectional
{
    pub fn new(maximum_streams: VariableLengthInteger) -> Self
    {
        Self {
            frame_type: VariableLengthInteger::from(FrameType::MaxStreamsUnidirectional),
            maximum_streams,
        }
    }
}
