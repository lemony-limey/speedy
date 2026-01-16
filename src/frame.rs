// This module contains the definitions for various types of frames that will be
// used during by QUIC.

use crate::frame::ack::Ack;
use crate::frame::ack_with_ecn::AckWithECN;
use crate::frame::connection_close_application_error::ConnectionCloseApplicationError;
use crate::frame::connection_close_success_or_quic_error::ConnectionCloseSuccessOrQuicError;
use crate::frame::crypto::Crypto;
use crate::frame::data_blocked::DataBlocked;
use crate::frame::handshake_done::HandshakeDone;
use crate::frame::max_data::MaxData;
use crate::frame::max_stream_data::MaxStreamData;
use crate::frame::max_streams_bidirectional::MaxStreamsBidirectional;
use crate::frame::max_streams_unidirectional::MaxStreamsUnidirectional;
use crate::frame::new_connection_id::NewConnectionID;
use crate::frame::new_token::NewToken;
use crate::frame::padding::Padding;
use crate::frame::path_challenge::PathChallenge;
use crate::frame::path_response::PathResponse;
use crate::frame::ping::Ping;
use crate::frame::reset_stream::ResetStream;
use crate::frame::retire_connection_id::RetireConnectionID;
use crate::frame::stop_sending::StopSending;
use crate::frame::stream::Stream;
use crate::frame::stream_data_blocked::StreamDataBlocked;
use crate::frame::streams_blocked_bidirectional::StreamsBlockedBidirectional;
use crate::frame::streams_blocked_unidirectional::StreamsBlockedUnidirectional;

pub mod ack;
pub mod ack_with_ecn;
pub mod connection_close_success_or_quic_error;
pub mod connection_close_application_error;
pub mod crypto;
pub mod data_blocked;
pub mod handshake_done;
pub mod max_data;
pub mod max_stream_data;
pub mod max_streams_bidirectional;
pub mod max_streams_unidirectional;
pub mod new_connection_id;
pub mod new_token;
pub mod padding;
pub mod path_challenge;
pub mod path_response;
pub mod ping;
pub mod reset_stream;
pub mod retire_connection_id;
pub mod stop_sending;
pub mod stream;
pub mod streams_blocked_bidirectional;
pub mod streams_blocked_unidirectional;
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
    StreamLen = 0x0A,
    StreamLenFin = 0x0B,
    StreamOff = 0x0C,
    StreamOffFin = 0x0D,
    StreamOffLen = 0x0E,
    StreamOffLenFin = 0x0F,
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

#[derive(Clone, Debug)]
pub enum Frame
{
    Padding(Padding),
    Ping(Ping),
    Ack(Ack),
    AckWithECN(AckWithECN),
    ResetStream(ResetStream),
    StopSending(StopSending),
    Crypto(Crypto),
    NewToken(NewToken),
    Stream(Stream),
    MaxData(MaxData),
    MaxStreamData(MaxStreamData),
    MaxStreamsBidirectional(MaxStreamsBidirectional),
    MaxStreamsUnidirectional(MaxStreamsUnidirectional),
    DataBlocked(DataBlocked),
    StreamDataBlocked(StreamDataBlocked),
    StreamsBlockedBidirectional(StreamsBlockedBidirectional),
    StreamsBlockedUnidirectional(StreamsBlockedUnidirectional),
    NewConnectionID(NewConnectionID),
    RetireConnectionID(RetireConnectionID),
    PathChallenge(PathChallenge),
    PathResponse(PathResponse),
    ConnectionCloseSuccessOrQuicError(ConnectionCloseSuccessOrQuicError),
    ConnectionCloseApplicationError(ConnectionCloseApplicationError),
    HandshakeDone(HandshakeDone),
}

#[cfg(test)]
mod tests
{

}
