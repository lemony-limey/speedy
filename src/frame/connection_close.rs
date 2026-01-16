use bytes::Bytes;
use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Debug)]
pub struct ConnectionClose
{
    frame_type:           FrameType,
    error_code:           VariableLengthInteger,
    error_frame_type:     Option<VariableLengthInteger>,
    reason_phrase_length: VariableLengthInteger,
    reason_phrase:        Bytes,
}

impl ConnectionClose
{
    pub fn new_success_or_quic_error(
        error_code:           VariableLengthInteger,
        error_frame_type:     Option<VariableLengthInteger>,
        reason_phrase_length: VariableLengthInteger,
        reason_phrase:        Bytes,
    ) -> Self
    {
        Self {
            frame_type: FrameType::ConnectionCloseSuccessOrQuicError,
            error_code,
            error_frame_type,
            reason_phrase_length,
            reason_phrase,
        }
    }

    pub fn new_application_error(
        error_code:           VariableLengthInteger,
        error_frame_type:     Option<VariableLengthInteger>,
        reason_phrase_length: VariableLengthInteger,
        reason_phrase:        Bytes,
    ) -> Self
    {
        Self {
            frame_type: FrameType::ConnectionCloseApplicationError,
            error_code,
            error_frame_type,
            reason_phrase_length,
            reason_phrase,
        }
    }
}
