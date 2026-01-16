use std::sync::Arc;
use crate::endpoint::Endpoint;

/// Connection IDs are used to identify connections so that endpoints can change during
/// connection migration without the connection being dropped.
#[derive(Clone, Debug)]
pub struct Connection
{
    endpoint: Arc<Endpoint>,
    connection_id: u64,
}
