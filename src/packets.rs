use serde::{Deserialize, Serialize};

#[repr(packed)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct LongHeaderPacket
{
    version:          u32,
    dest_conn_id_len: u8,
    dest_conn_id:     [u8; 20],
    src_conn_id_len:  u8,
    src_conn_id:      [u8; 20],
    #[serde(flatten)]
    payload: Payload,
}

#[repr(packed)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct ShortHeaderPacket
{
    payload: Payload,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
enum Payload
{
    Initial {
        // token length
    },
}