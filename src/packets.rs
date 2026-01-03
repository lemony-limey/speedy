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
    dest_conn_id:     [u8; 20],
    packet_number: u32,
}

// #[derive(Clone, Debug)]
// struct VersionNegotiationHeader
// {
//
// }

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
