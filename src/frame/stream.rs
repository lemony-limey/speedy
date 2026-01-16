use bytes::Bytes;
use crate::frame::FrameType;
use crate::stream::StreamID;
use crate::variable_length_integer::VariableLengthInteger;

/// Indicates whether the Offset field present in this frame.
/// The OFF bit is 0x04.
#[derive(Clone, Debug)]
pub struct Off(pub bool);

/// Indicates whether the Length field is present in this frame.
/// The LEN bit is 0x02.
#[derive(Clone, Debug)]
pub struct Len(pub bool);

/// Indicates whether this frame marks the end of the stream.
/// The FIN bit is 0x01.
#[derive(Clone, Debug)]
pub struct Fin(pub bool);

#[derive(Clone, Debug)]
pub struct Stream
{
    frame_type: VariableLengthInteger,
    off:        Off,
    len:        Len,
    fin:        Fin,
    stream_id:  StreamID,
    // Only present if the OFF bit is set
    offset:     Option<VariableLengthInteger>,
    // Only present if the LEN bit is set
    length:     Option<VariableLengthInteger>,
    data:       Bytes,
}

impl Stream
{
    pub fn new(
        off:       Off,
        len:       Len,
        fin:       Fin,
        stream_id: StreamID,
        offset:    Option<VariableLengthInteger>,
        length:    Option<VariableLengthInteger>,
        data:      Bytes,
    ) -> Self
    {
        // Frame type depends on the values of off, len and fin.
        let frame_type = match (&off, &len, &fin) {
            (Off(false), Len(false), Fin(false)) => FrameType::StreamNoneSet,
            (Off(false), Len(false), Fin(true))  => FrameType::StreamFin,
            (Off(false), Len(true),  Fin(false)) => FrameType::StreamLen,
            (Off(false), Len(true),  Fin(true))  => FrameType::StreamLenFin,
            (Off(true),  Len(false), Fin(false)) => FrameType::StreamOff,
            (Off(true),  Len(false), Fin(true))  => FrameType::StreamOffFin,
            (Off(true),  Len(true),  Fin(false)) => FrameType::StreamOffLen,
            (Off(true),  Len(true),  Fin(true))  => FrameType::StreamOffLenFin,
        };

        Self {
            frame_type: VariableLengthInteger::from(frame_type),
            off,
            len,
            fin,
            stream_id,
            offset,
            length,
            data,
        }
    }
}
