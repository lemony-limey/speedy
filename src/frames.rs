// This module contains the definitions for various types of frames that will be
// used during by QUIC.

enum FrameType
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



enum Frame
{
    
}