use crate::frame::FrameType;
use crate::quic_stream::StreamID;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Copy, Debug)]
pub struct StreamDataBlocked
{
    frame_type:          VariableLengthInteger,
    stream_id:           StreamID,
    maximum_stream_data: VariableLengthInteger,
}

impl StreamDataBlocked
{
    pub fn new(
        stream_id:           StreamID,
        maximum_stream_data: VariableLengthInteger,
    ) -> Self
    {
        Self {
            frame_type: VariableLengthInteger::from(FrameType::StreamDataBlocked),
            stream_id,
            maximum_stream_data,
        }
    }
}
