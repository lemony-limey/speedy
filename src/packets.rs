use crate::frames::Frame;
use crate::VariableLengthInteger;

#[derive(Clone, Debug)]
struct LongHeader
{
    reserved_and_packet_number_length: u8,
    version:          u32,
    dest_conn_id_len: u8,
    dest_conn_id:     [u8; 20],
    src_conn_id_len:  u8,
    src_conn_id:      [u8; 20],
}

#[derive(Clone, Debug)]
struct ShortHeader
{
    reserved_and_packet_number_length: u8,
    dest_conn_id:  [u8; 20],
    packet_number: u32,
}

#[derive(Clone, Debug)]
enum Packet
{
    VersionNegotiation
    {
        form_and_unused:   u8,
        version:           u32,  // Must be set to 0
        dest_conn_id_len:  u8,
        dest_conn_id:      [u8; 255],
        src_conn_id_len:   u8,
        src_conn_id:       [u8; 255],
        supported_version: u32,  //
    },
    Initial  // 0x00
    {
        header: LongHeader,
        token_length: VariableLengthInteger,

    },
    ZeroRTT  // 0x01
    {
        header: LongHeader,

    },
    Handshake  // 0x02
    {
        header: LongHeader,

    },
    Retry  // 0x03
    {
        header: LongHeader,

    },
    OneRTT  // Standard packet basically
    {
        header: ShortHeader,
        frames: Vec<Frame>,
    },
}

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
