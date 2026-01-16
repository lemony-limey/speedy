use std::sync::Arc;
use crate::endpoint::Endpoint;

#[derive(Clone, Debug)]
pub struct Connection
{
    endpoint: Arc<Endpoint>,
    connection_id: u64,
}
