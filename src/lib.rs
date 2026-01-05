use std::io;
use anyhow::anyhow;

mod frames;
mod packets;
mod parser;
mod quic_listener;
mod quic_socket;
mod quic_stream;
pub mod variable_length_integer;

