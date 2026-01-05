use clap::Parser;
use std::net::{
    Ipv4Addr,
    UdpSocket,
};
use std::str::from_utf8;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args
{
    /// The local port number that the server should attempt to bind to.
    #[arg(short, long)]
    port_number: u16,
}

#[tokio::main]
async fn main() -> std::io::Result<()>
{
    // Parse the command-line arguments
    let args = Args::parse();

    // Ensure port number is within range (i.e., not a well-known value)
    if args.port_number < 1024
    {
        panic!("port number must be strictly greater than 1023");
    }

    // Attempt to convert the host name and port number to an address
    let socket_address = format!("{}:{}", Ipv4Addr::LOCALHOST, args.port_number);
    // TODO: Use let-else

    // Bind the socket to the remote server's address
    let socket = UdpSocket::bind(&socket_address)
        .expect(&format!("could not bind socket to the local address: {}", socket_address));
    println!("bound to {}", socket.local_addr()?);

    // Listen for an incoming connection
    // (Multiple connections will be accepted in the next server version)
    println!("listening...");
    let mut buffer = [0u8; 1500];
    let (number_of_bytes_received, src_addr) = socket.recv_from(&mut buffer)?;

    let received_string = from_utf8(&buffer[..number_of_bytes_received])
        .expect("could not convert received data into a UTF-8 string");
    println!("{} -> {}, received {} bytes: {}", src_addr, socket_address, number_of_bytes_received, received_string);

    let number_of_bytes_sent = socket.send_to("Hello client!".as_bytes(), src_addr)
        .expect("could not send data");
    println!("{} -> {}, sent {} bytes: {}", socket_address, src_addr, number_of_bytes_sent, "Hello client!");

    Ok(())
}