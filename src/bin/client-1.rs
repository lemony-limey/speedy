use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::str::from_utf8;
use anyhow::Context;
use clap::Parser;

/// A struct to represent the expected command-line arguments.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args
{
    /// The host server that we should try to connect to.
    #[arg(long)]
    host_name: String,
    
    /// The port number for the remote server that we wish to connect to.
    #[arg(short, long)]
    port_number: u16,
}

fn main() -> anyhow::Result<()>
{
    // Parse the command-line arguments
    let args = Args::parse();

    // Ensure port number is within range (i.e., not a well-known value)
    if args.port_number < 1024
    {
        panic!("Port number cannot be a well-known value (i.e. between 0 and 1023, inclusive of both)");
    }
    
    // Construct the server's address using the command-line arguments
    let server_addr = format!("{}:{}", args.host_name, args.port_number)
        .to_socket_addrs()
        .with_context(|| "invalid arguments supplied")?
        .next()
        .with_context(|| "could not convert arguments to a valid socket address")?;

    // Bind the local socket to an ephemeral port
    let socket_address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0));

    // Bind the socket to the local client's address
    let socket = UdpSocket::bind(&socket_address)
        .expect("Could not bind socket to remote server's address");

    // Send data to the server
    let number_of_bytes_sent = socket.send_to("Hello server!".as_bytes(), server_addr)
        .with_context(|| "could not send data from client to server")?;
    println!(
        "{} -> {}, sent {} bytes: {}", 
        socket.local_addr()
            .with_context(|| "could not get local socket's local address")?, 
        server_addr, 
        number_of_bytes_sent, 
        "Hello server!",
    );

    // Receive data from the server
    let mut buffer = [0u8; 1500];
    let (number_of_bytes_received, src_addr) = socket.recv_from(&mut buffer)
        .with_context(|| "could not receive data from server")?;
    let received_string = from_utf8(&buffer[..number_of_bytes_received])
        .expect("could not convert received data into a UTF-8 string");
    println!(
        "{} -> {}, received {} bytes: {}", 
        src_addr, 
        socket.local_addr()
            .with_context(|| "could not get local socket's local address")?, 
        number_of_bytes_received, 
        received_string,
    );

    Ok(())
}