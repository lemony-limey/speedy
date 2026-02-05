use std::sync::Arc;
use crate::endpoint::Endpoint;

/// Connection IDs are used to identify connections so that endpoints can change during
/// connection migration without the connection being dropped.
#[derive(Clone, Debug)]
pub struct Connection
{
    endpoint: Arc<Endpoint>,
    connection_id: Option<u64>,
    pub connection_id_length: Option<u8>,
}

impl Connection
{
    pub fn new(
        endpoint:             Arc<Endpoint>,
        connection_id:        Option<u64>,
        connection_id_length: Option<u8>,
    ) -> Self
    {
        Self
        {
            endpoint,
            connection_id,
            connection_id_length,
        }
    }
}
