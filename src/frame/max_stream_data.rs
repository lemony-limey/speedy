use crate::frame::FrameType;
use crate::quic_stream::StreamID;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Debug)]
pub struct MaxStreamData
{
    frame_type:          VariableLengthInteger,
    stream_id:           StreamID,
    maximum_stream_data: VariableLengthInteger,
}

impl MaxStreamData
{
    pub fn new(
        stream_id:           StreamID,
        maximum_stream_data: VariableLengthInteger,
    ) -> Self
    {
        Self {
            frame_type: VariableLengthInteger::from(FrameType::MaxStreamData),
            stream_id,
            maximum_stream_data,
        }
    }
}
