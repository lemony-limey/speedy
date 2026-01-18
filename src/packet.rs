use bytes::Bytes;
use crate::frame::Frame;
use crate::variable_length_integer::VariableLengthInteger;

pub enum Header
{
    Long(LongHeader),
    Short(ShortHeader),
}

#[derive(Clone, Debug)]
pub struct LongHeader
{
    reserved_and_packet_number_length: u8,
    version:                           u32,
    dest_conn_id_len:                  u8,
    dest_conn_id:                      Bytes,  // At most 20 bytes
    src_conn_id_len:                   u8,
    src_conn_id:                       Bytes,  // At most 20 bytes
}

#[derive(Clone, Debug)]
pub struct ShortHeader
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
        header:       LongHeader,
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

pub(crate) enum PacketType
{
    Initial   = 0x00,
    ZeroRTT   = 0x01,
    Handshake = 0x02,
    Retry     = 0x03,
    VersionNegotiation,
}

impl PacketType
{
    pub(crate) fn get_type_from_u8(value: u8) -> Self
    {
        match value
        {
            0x00 => PacketType::Initial,
            0x01 => PacketType::ZeroRTT,
            0x02 => PacketType::Handshake,
            0x03 => PacketType::Retry,
            _    => PacketType::VersionNegotiation,
        }
    }
}

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

/// HandshakeHeader is defined separately because it has different fields to both the short and
/// long headers.
struct HandshakeHeader
{
    type_and_length: u32,  // First byte is message type
    // Next 3 bytes are length of ClientHello data
}

/// The serialise method takes a Packet and serialises it into bytes.
pub trait PacketSerialise
{
    fn serialise(packet: Packet) -> Bytes;
}

/// The serialise method takes an incoming byte stream and deserialises it into a packet.
pub trait PacketDeserialise
{
    fn serialise(bytes: Bytes) -> Packet;
}
