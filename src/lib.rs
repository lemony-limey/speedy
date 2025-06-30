mod frames;
mod packets;
mod tls;

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