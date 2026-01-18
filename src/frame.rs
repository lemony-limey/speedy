// This module contains the definitions for various types of frames that will be
// used during by QUIC.

use crate::frame::ack::Ack;
use crate::frame::connection_close::ConnectionClose;
use crate::frame::crypto::Crypto;
use crate::frame::data_blocked::DataBlocked;
use crate::frame::max_data::MaxData;
use crate::frame::max_stream_data::MaxStreamData;
use crate::frame::max_streams::MaxStreams;
use crate::frame::new_connection_id::NewConnectionID;
use crate::frame::new_token::NewToken;
use crate::frame::path_challenge::PathChallenge;
use crate::frame::path_response::PathResponse;
use crate::frame::reset_stream::ResetStream;
use crate::frame::retire_connection_id::RetireConnectionID;
use crate::frame::stop_sending::StopSending;
use crate::frame::stream::Stream;
use crate::frame::stream_data_blocked::StreamDataBlocked;
use crate::frame::streams_blocked::StreamsBlocked;

pub mod ack;
pub mod connection_close;
pub mod crypto;
pub mod data_blocked;
pub mod max_data;
pub mod max_stream_data;
pub mod max_streams;
pub mod new_connection_id;
pub mod new_token;
pub mod path_challenge;
pub mod path_response;
pub mod reset_stream;
pub mod retire_connection_id;
pub mod stop_sending;
pub mod stream;
pub mod streams_blocked;
pub mod stream_data_blocked;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum FrameType
{
    Padding = 0x00,
    Ping = 0x01,
    Ack = 0x02,
    AckWithECN = 0x03,
    ResetStream = 0x04,
    StopSending = 0x05,
    Crypto = 0x06,
    NewToken = 0x07,
    // Stream types are some value in (0x08 ..= 0x0f) depending on whether
    // any of the OFF (0x04), LEN (0x02) and FIN (0x01) bit flags are set.
    StreamNoneSet = 0x08,
    StreamFin = 0x09,
    StreamLen = 0x0a,
    StreamLenFin = 0x0b,
    StreamOff = 0x0c,
    StreamOffFin = 0x0d,
    StreamOffLen = 0x0e,
    StreamOffLenFin = 0x0f,
    MaxData = 0x10,
    MaxStreamData = 0x11,
    MaxStreamsBidirectional = 0x12,
    MaxStreamsUnidirectional = 0x13,
    DataBlocked = 0x14,
    StreamDataBlocked = 0x15,
    StreamsBlockedBidirectional = 0x16,
    StreamsBlockedUnidirectional = 0x17,
    NewConnectionID = 0x18,
    RetireConnectionID = 0x19,
    PathChallenge = 0x1a,
    PathResponse = 0x1b,
    ConnectionCloseSuccessOrQuicError = 0x1c,
    ConnectionCloseApplicationError = 0x1d,
    HandshakeDone = 0x1e,
}

impl PartialEq<u8> for FrameType
{
    fn eq(&self, other: &u8) -> bool
    {
        (*self as u8).eq(other)
    }
}

impl PartialEq<FrameType> for u8
{
    fn eq(&self, other: &FrameType) -> bool
    {
        self.eq(&(*other as u8))
    }
}



impl FrameType
{
    pub(crate) fn u8_to_frame_type(value: u8) -> Self
    {
        match value
        {
            0x00 => Self::Padding,
            0x01 => Self::Ping,
            0x02 => Self::Ack,
            0x03 => Self::AckWithECN,
            0x04 => Self::ResetStream,
            0x05 => Self::StopSending,
            0x06 => Self::Crypto,
            0x07 => Self::NewToken,
            0x08 => Self::StreamNoneSet,
            0x09 => Self::StreamFin,
            0x0a => Self::StreamLen,
            0x0b => Self::StreamLenFin,
            0x0c => Self::StreamOff,
            0x0d => Self::StreamOffFin,
            0x0e => Self::StreamOffLen,
            0x0f => Self::StreamOffLenFin,
            0x10 => Self::MaxData,
            0x11 => Self::MaxStreamData,
            0x12 => Self::MaxStreamsBidirectional,
            0x13 => Self::MaxStreamsUnidirectional,
            0x14 => Self::DataBlocked,
            0x15 => Self::StreamDataBlocked,
            0x16 => Self::StreamsBlockedBidirectional,
            0x17 => Self::StreamsBlockedUnidirectional,
            0x18 => Self::NewConnectionID,
            0x19 => Self::RetireConnectionID,
            0x1a => Self::PathChallenge,
            0x1b => Self::PathResponse,
            0x1c => Self::ConnectionCloseSuccessOrQuicError,
            0x1d => Self::ConnectionCloseApplicationError,
            0x1e => Self::HandshakeDone,
            31_u8..=u8::MAX => unimplemented!(),
        }
    }
}


#[derive(Clone, Debug)]
pub enum Frame
{
    Padding,
    Ping,
    Ack(Ack),
    AckWithECN(Ack),
    ResetStream(ResetStream),
    StopSending(StopSending),
    Crypto(Crypto),
    NewToken(NewToken),
    Stream(Stream),
    MaxData(MaxData),
    MaxStreamData(MaxStreamData),
    MaxStreamsBidirectional(MaxStreams),
    MaxStreamsUnidirectional(MaxStreams),
    DataBlocked(DataBlocked),
    StreamDataBlocked(StreamDataBlocked),
    StreamsBlockedBidirectional(StreamsBlocked),
    StreamsBlockedUnidirectional(StreamsBlocked),
    NewConnectionID(NewConnectionID),
    RetireConnectionID(RetireConnectionID),
    PathChallenge(PathChallenge),
    PathResponse(PathResponse),
    ConnectionCloseSuccessOrQuicError(ConnectionClose),
    ConnectionCloseApplicationError(ConnectionClose),
    HandshakeDone,
}


#[cfg(test)]
mod tests
{

}
