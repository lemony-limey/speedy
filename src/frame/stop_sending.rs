use crate::frame::FrameType;
use crate::stream::StreamID;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Copy, Debug)]
pub struct StopSending
{
    frame_type:                      FrameType,
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
            frame_type: FrameType::StopSending,
            stream_id,
            application_protocol_error_code,
        }
    }
}
