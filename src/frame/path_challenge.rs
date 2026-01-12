use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

/// Used to check reachability to the peer and for path validation during connection migration.
/// (RFC 9000, Section 19.17).
#[derive(Clone, Copy, Debug)]
pub struct PathChallenge
{
    frame_type: VariableLengthInteger,
    data:       u64,
}

impl PathChallenge
{
    pub fn new(data: u64) -> Self
    {
        Self {
            frame_type: VariableLengthInteger::from(FrameType::PathChallenge),
            data,
        }
    }
}
