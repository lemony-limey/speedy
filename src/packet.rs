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
    // The first (i.e. most significant) 4 bits will be 0, as they will already have been parsed.
    // During serialisation, the first four bits will be filled in with the necessary values.
    type_specific_bits: u8,
    version:            u32,
    dest_conn_id_len:   u8,
    dest_conn_id:       Bytes,  // At most 20 bytes
    src_conn_id_len:    u8,
    src_conn_id:        Bytes,  // At most 20 bytes
}

#[derive(Clone, Debug)]
pub struct ShortHeader
{
    spin_bit:             bool,
    key_phase:            bool,
    packet_number_length: u8,
    dest_conn_id:         Bytes,  // At most 20 bytes
    packet_number:        u32,
}

#[derive(Clone, Debug)]
pub enum Packet
{
    VersionNegotiation  // Only ever sent by servers
    {
        form_and_unused:   u8,
        version:           u32,   // Must be set to 0
        dest_conn_id_len:  u8,
        dest_conn_id:      Bytes, // At most 255 bytes
        src_conn_id_len:   u8,
        src_conn_id:       Bytes, // At most 255 bytes
        supported_version: u32,
    },
    /// RFC 9000 Section 17.2.2.1:
    /// A client stops both sending and processing Initial packets when
    /// it sends its first Handshake packet.
    /// A server stops sending and processing Initial packets when it receives its first Handshake
    /// packet.
    ///
    /// Payload of an Initial packet:
    ///     - One or more CRYPTO frames and/or one or more ACK frames
    ///     - 0 or more PING frames
    ///     - 0 or more PADDING frames
    ///     - 0 or more CONNECTION_CLOSE frames of type 0x1c (success or QUIC error)
    Initial  // 0x00
    {
        header:         LongHeader,
        token_length:   VariableLengthInteger,
        token:          Bytes,
        length:         VariableLengthInteger,
        packet_number:  u32,
        packet_payload: Vec<Frame>,
    },
    ZeroRTT  // 0x01
    {
        header:         LongHeader,
        length:         VariableLengthInteger,
        packet_number:  u32,
        packet_payload: Vec<Frame>,
    },
    Handshake  // 0x02
    {
        header:         LongHeader,
        /// Contains Crypto frames, may contain Ping, Padding,
        /// Ack or ConnectionClose frames also.
        /// Anything else should result in a ProtocolViolation error.
        length:         VariableLengthInteger,
        packet_number:  u32,
        packet_payload: Vec<Frame>,
    },
    /// Only one Retry packet is processed for each connection attempt. Subsequent ones are
    /// discarded.
    /// 17.2.5.2: Retry packets do not contain packet numbers and so cannot be explicitly acknowledged.
    Retry  // 0x03
    {
        header:              LongHeader,
        retry_token:         Bytes,
        retry_integrity_tag: u128,
    },
    OneRTT  // Standard packet basically
    {
        header: ShortHeader,
        frames: Vec<Frame>,
    },
}

impl Packet
{
    /// Serialise recursively serialises all child elements of the packet in a sort of DFS-esque
    /// fashion.
    pub(crate) fn serialise(&self) -> Bytes
    {
        todo!()
    }
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
