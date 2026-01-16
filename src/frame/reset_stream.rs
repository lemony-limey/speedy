use crate::frame::FrameType;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Copy, Debug)]
pub struct ResetStream
{
    frame_type:                      FrameType,
    stream_id:                       VariableLengthInteger,
    application_protocol_error_code: VariableLengthInteger,
    final_size:                      VariableLengthInteger,
}

impl ResetStream
{
    pub fn new(
        stream_id:                       VariableLengthInteger,
        application_protocol_error_code: VariableLengthInteger,
        final_size:                      VariableLengthInteger,
    ) -> Self
    {
        Self {
            frame_type: FrameType::ResetStream,
            stream_id,
            application_protocol_error_code,
            final_size,
        }
    }
}
