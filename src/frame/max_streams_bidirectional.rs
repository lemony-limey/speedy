use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

/// MaxStreamsBidirectional represents the count of the cumulative number of [bidirectional]
/// streams ... that can be opened over the lifetime of the connection (RFC 9000, Section 19.11).
#[derive(Clone, Copy, Debug)]
pub struct MaxStreamsBidirectional
{
    // Bidirectional: 0x12
    frame_type:      VariableLengthInteger,
    maximum_streams: VariableLengthInteger,
}

impl MaxStreamsBidirectional
{
    pub fn new(maximum_streams: VariableLengthInteger) -> Self
    {
        Self {
            frame_type: VariableLengthInteger::from(FrameType::MaxStreamsBidirectional),
            maximum_streams,
        }
    }
}
