// This module contains the definitions for various types of frames that will be
// used during by QUIC.

use bytes::Bytes;
use crate::quic_stream::StreamID;
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
    ConnectionCloseNoErrorOrQuicError = 0x1c,
    ConnectionCloseApplicationError = 0x1d,
    HandshakeDone = 0x1e,
}

#[derive(Clone, Debug)]
pub enum Frame
{
    Padding, // type: 0x00, No content
    Ping,    // type: 0x01, No content
    Ack      // type: 0x02
    {
        largest_acknowledged: VariableLengthInteger,
        ack_delay:            VariableLengthInteger,  // Delay in microseconds (us).
        ack_range_count:      VariableLengthInteger,  // Number of ACK range fields in the frame.
        first_ack_range:      VariableLengthInteger,
        ack_ranges:           Option<Vec<AckRange>>,  // Length of Vec given by ack_range_count field
    },
    AckWithECN  // type: 0x03
    {
        largest_acknowledged: VariableLengthInteger,
        ack_delay:            VariableLengthInteger,  // Delay in microseconds (us).
        ack_range_count:      VariableLengthInteger,  // Number of ACK range fields in the frame.
        first_ack_range:      VariableLengthInteger,
        ack_ranges:           Option<Vec<AckRange>>,  // Length of Vec given by ack_range_count field
    },
    ResetStream  // type: 0x04
    {
        stream_id:                       VariableLengthInteger,
        application_protocol_error_code: VariableLengthInteger,
        final_size:                      VariableLengthInteger,
    },
    StopSending  // type: 0x05
    {
        stream_id:                       StreamID,
        application_protocol_error_code: VariableLengthInteger,
    },
    // Crypto frames cannot be sent in 0-RTT packets.
    Crypto  // type: 0x06
    {
        offset: VariableLengthInteger,
        length: VariableLengthInteger,
        data:   Bytes,
    },
    NewToken  // type: 0x07
    {
        // Length of the token in bytes
        token_length: VariableLengthInteger,
        token:        Bytes,
    },
    Stream  // type: (0x08..=0x0f)
    {
        // Indicates that there is an Offset field present in this frame.
        // The OFF bit is 0x04
        off:       bool,
        // Indicates that the Length field is present in this frame.
        // The LEN bit is 0x02
        len:       bool,
        // Indicates that this frame marks the end of the stream.
        // The FIN bit is 0x01
        fin:       bool,
        stream_id: StreamID,
        // Only present if the OFF bit is set
        offset:    Option<VariableLengthInteger>,
        // Only present if the LEN bit is set
        length:    Option<VariableLengthInteger>,
        data:      Bytes,
    },
    MaxData  // type: 0x10
    {
        maximum_data: VariableLengthInteger,
    },
    // Can be sent for streams in the "Recv" state.
    MaxStreamData  // type: 0x11
    {
        stream_id:           StreamID,
        maximum_stream_data: VariableLengthInteger,
    },
    // MaxStreamsBidirectional represents the count of the cumulative number of [bidirectional]
    // streams ... that can be opened over the lifetime of the connection (RFC 9000, Section 19.11).
    MaxStreamsBidirectional  // type: 0x12
    {
        maximum_streams: VariableLengthInteger,
    },
    // MaxStreamsUnidirectional represents the count of the cumulative number of [unidirectional]
    // streams ... that can be opened over the lifetime of the connection (RFC 9000, Section 19.11).
    MaxStreamsUnidirectional  // type: 0x13
    {
        maximum_streams: VariableLengthInteger,
    },
    DataBlocked  // type: 0x14
    {
        // The connection-level limit at which blocking occurred.
        maximum_data: VariableLengthInteger,
    },
    StreamDataBlocked  // type: 0x15
    {
        stream_id:           StreamID,
        maximum_stream_data: VariableLengthInteger,
    },
    StreamsBlockedBidirectional  // type: 0x16
    {
        maximum_streams: VariableLengthInteger,
    },
    StreamsBlockedUnidirectional  // type: 0x17
    {
        maximum_streams: VariableLengthInteger,
    },
    NewConnectionID  // type: 0x18
    {
        sequence_number:       VariableLengthInteger,
        retire_prior_to:       VariableLengthInteger,
        length:                u8,
        connection_id:         Bytes,  // At most 20 bytes
        stateless_reset_token: u128,
    },
    RetireConnectionID  // type: 0x19
    {
        sequence_number: VariableLengthInteger,
    },
    // Used to check reachability to the peer and for path validation during connection migration.
    // (RFC 9000, Section 19.17).
    PathChallenge  // type: 0x1a
    {
        data: u64,
    },
    PathResponse  // type: 0x1b
    {
        data: u64,
    },
    ConnectionCloseNoErrorOrQuicError  // type: 0x1c
    {
        error_code:           VariableLengthInteger,
        frame_type:           Option<VariableLengthInteger>,
        reason_phrase_length: VariableLengthInteger,
        reason_phrase:        Bytes,
    },
    ConnectionCloseApplicationError  // type: 0x1d
    {
        error_code:           VariableLengthInteger,
        frame_type:           Option<VariableLengthInteger>,
        reason_phrase_length: VariableLengthInteger,
        reason_phrase:        Bytes,
    },
    HandshakeDone,  // type: 0x1e
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
