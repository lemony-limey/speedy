use bytes::Bytes;
use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Debug)]
pub struct ConnectionCloseApplicationError
{
    frame_type:           VariableLengthInteger,
    error_code:           VariableLengthInteger,
    error_frame_type:     Option<VariableLengthInteger>,
    reason_phrase_length: VariableLengthInteger,
    reason_phrase:        Bytes,
}

impl ConnectionCloseApplicationError
{
    pub fn new(
        error_code:           VariableLengthInteger,
        error_frame_type:     Option<VariableLengthInteger>,
        reason_phrase_length: VariableLengthInteger,
        reason_phrase:        Bytes,
    ) -> Self
    {
        Self {
            frame_type: VariableLengthInteger::from(FrameType::ConnectionCloseApplicationError),
            error_code,
            error_frame_type,
            reason_phrase_length,
            reason_phrase,
        }
    }
}
