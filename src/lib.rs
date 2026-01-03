mod frames;
mod packets;
mod parser;
mod quic_listener;
mod quic_socket;
mod quic_stream;

enum TransportErrorCodes
{
    NoError = 0x00,
    InternalError = 0x01,
    ConnectionRefused = 0x02,
    FlowControlError = 0x03,
    StreamLimitError = 0x04,
    StreamStateError = 0x05,
    FinalSizeError = 0x06,
    FrameEncodingError = 0x07,
    TransportParameterError = 0x08,
    ConnectionIDLimitError = 0x09,
    ProtocolViolation = 0x0a,
    InvalidToken = 0x0b,
    ApplicationError = 0x0c,
    CryptoBufferExceeded = 0x0d,
    KeyUpdateError = 0x0e,
    AEADLimitReached = 0x0f,
    NoViablePath = 0x10,
    CryptoError = 0x0100,  // ..=0x01ff. Cryptographic Handshake failed.
}

#[repr(C)]
enum HandshakeType
{
    ClientHello = 1,
    ServerHello = 2,
    NewSessionTicket = 4,
    EndOfEarlyData = 5,
    EncryptedExtensions = 8,
    Certificate = 11,
    CertificateRequest = 13,
    CertificateVerify = 15,
    Finished = 20,
    KeyUpdate = 24,
    MessageHash = 254,
}

#[repr(C)]
struct HandshakeHeader
{
    type_and_length: u32,  // First byte is message type
    // Next 3 bytes are length of ClientHello data
}


#[derive(Clone, Debug)]
pub enum VariableLengthInteger
{
    EightBit(u8),      // 00
    SixteenBit(u16),   // 01
    ThirtyTwoBit(u32), // 10
    SixtyFourBit(u64), // 11
}

