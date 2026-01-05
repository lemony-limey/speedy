// This module contains the definitions for various types of frames that will be
// used during by QUIC.

use crate::variable_length_integer::VariableLengthInteger;

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
    // Stream = some value in (0x08 ..= 0x0f) depending on OFF, LEN and FIN bit flags
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
    ConnectionClose = 0x1c,
    ConnectionCloseApplicationError = 0x1d,
    HandshakeDone = 0x1e,
}

#[derive(Clone, Debug)]
pub enum Frame
{
    Padding, // 0x00, No content
    Ping,    // 0x01, No content
    Ack  // 0x02
    {
        largest_acknowledged: VariableLengthInteger,
        ack_delay:            VariableLengthInteger,  // Delay in microseconds (us).
        ack_range_count:      VariableLengthInteger,  // Number of ACK range fields in the frame.
        first_ack_range:      VariableLengthInteger,
        ack_ranges:           Option<Vec<AckRange>>,  // Length of Vec given by ack_range_count field
    },
    AckWithECN  // 0x03
    {
        largest_acknowledged: VariableLengthInteger,
        ack_delay:            VariableLengthInteger,  // Delay in microseconds (us).
        ack_range_count:      VariableLengthInteger,  // Number of ACK range fields in the frame.
        first_ack_range:      VariableLengthInteger,
        ack_ranges:           Option<Vec<AckRange>>,  // Length of Vec given by ack_range_count field
    },
    ResetStream  // 0x04
    {
        stream_id: VariableLengthInteger,

    },
}

#[derive(Copy, Clone, Debug)]
pub struct AckRange
{
    gap:              VariableLengthInteger,
    ack_range_length: VariableLengthInteger,
}

#[derive(Copy, Clone, Debug)]
pub struct ECNCounts
{
    ect0_count:   VariableLengthInteger,
    ect1_count:   VariableLengthInteger,
    ecn_ce_count: VariableLengthInteger,
}
