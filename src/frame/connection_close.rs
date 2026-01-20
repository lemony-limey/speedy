use bytes::Bytes;
use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Debug)]
pub struct ConnectionClose
{
    // Success/QUIC Error: 0x1c
    // Application Error:  0x1d
    frame_type:           FrameType,
    error_code:           VariableLengthInteger,
    // error_frame_type is None if frame_type is 0x1d (application error)
    error_frame_type:     Option<VariableLengthInteger>,
    reason_phrase_length: VariableLengthInteger,
    reason_phrase:        Bytes,
}

impl ConnectionClose
{
    pub fn new(
        frame_type:           FrameType,
        error_code:           VariableLengthInteger,
        error_frame_type:     Option<VariableLengthInteger>,
        reason_phrase_length: VariableLengthInteger,
        reason_phrase:        Bytes,
    ) -> Self
    {
        // ConnectionCloseApplicationError does not include an
        // error frame type field.
        assert!(
            !(frame_type == FrameType::ConnectionCloseApplicationError
                && error_frame_type.is_some())
        );

        Self {
            frame_type,
            error_code,
            error_frame_type,
            reason_phrase_length,
            reason_phrase,
        }
    }
}
