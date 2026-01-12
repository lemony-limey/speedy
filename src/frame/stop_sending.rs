use crate::frame::FrameType;
use crate::quic_stream::StreamID;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Copy, Debug)]
pub struct StopSending
{
    frame_type:                      VariableLengthInteger,
    stream_id:                       StreamID,
    application_protocol_error_code: VariableLengthInteger,
}

impl StopSending
{
    pub fn new(
        stream_id:                       StreamID,
        application_protocol_error_code: VariableLengthInteger,
    ) -> Self
    {
        Self {
            frame_type: VariableLengthInteger::from(FrameType::StopSending),
            stream_id,
            application_protocol_error_code,
        }
    }
}
