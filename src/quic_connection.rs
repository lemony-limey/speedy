use std::sync::Arc;
use crate::quic_endpoint::QuicEndpoint;

#[derive(Clone, Debug)]
pub struct QuicConnection
{
    endpoint: Arc<QuicEndpoint>,
    connection_id: u64,
}
