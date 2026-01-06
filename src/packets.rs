use bytes::Bytes;
use crate::frames::Frame;
use crate::variable_length_integer::VariableLengthInteger;

#[derive(Clone, Debug)]
struct LongHeader
{
    reserved_and_packet_number_length: u8,
    version:          u32,
    dest_conn_id_len: u8,
    dest_conn_id:     Bytes,  // At most 20 bytes
    src_conn_id_len:  u8,
    src_conn_id:      Bytes,  // At most 20 bytes
}

#[derive(Clone, Debug)]
struct ShortHeader
{
    reserved_and_packet_number_length: u8,
    dest_conn_id:                      Bytes,  // At most 20 bytes
    packet_number:                     u32,
}

#[derive(Clone, Debug)]
pub enum Packet
{
    VersionNegotiation
    {
        form_and_unused:   u8,
        version:           u32,   // Must be set to 0
        dest_conn_id_len:  u8,
        dest_conn_id:      Bytes, // At most 255 bytes
        src_conn_id_len:   u8,
        src_conn_id:       Bytes, // At most 255 bytes
        supported_version: u32,
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
