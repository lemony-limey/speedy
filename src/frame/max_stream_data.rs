use crate::frame::FrameType;
use crate::stream::StreamID;
use crate::variable_length_integer::VariableLengthInteger;

/// Can be sent for streams in the "Recv" state.
#[derive(Clone, Debug)]
pub struct MaxStreamData
{
    frame_type:          FrameType,
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
            frame_type: FrameType::MaxStreamData,
            stream_id,
            maximum_stream_data,
        }
    }
}
