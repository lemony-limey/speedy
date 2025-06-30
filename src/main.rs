use std::net::UdpSocket;
use rand::{RngCore};
use ring::{
    agreement,
    digest::{
        SHA256_OUTPUT_LEN
    },
    hkdf::{
        HKDF_SHA256,
        Salt,
    },
};

/// The value of the very first SHA-1 collision, used here for generating encryption keys.
const INITIAL_ENCRYPTION_KEY_SALT: &str = "38762cf7f55934b34d179ae6a4c80cadccbb7f0a";

fn main()
{
    // Server Key Exchange Generation

    let rng = ring::rand::SystemRandom::new();

    // X25519 Key Generation
    let private_key = agreement::EphemeralPrivateKey::generate(
        &agreement::X25519,
        &rng,
    ).expect("Could not generate ephemeral private key");
    let public_key = private_key
        .compute_public_key()
        .expect("Could not compute public key from private key");

    // Server Initial Keys Calculation

    // Generate 8 bytes of random data
    let mut initial_random = [0u8; 8];
    rand::rng().fill_bytes(&mut initial_random);

    // Generate the initial secret
    let salt = Salt::new(HKDF_SHA256, INITIAL_ENCRYPTION_KEY_SALT.as_bytes());
    let initial_secret = salt.extract(&initial_random);

    // Derive the client and server secrets
    // The following source was consulted when investigating how to implement this.
    // https://web3developer.io/deriving-cryptographic-keys-with-hkdf-in-rust-using-ring/

    // Client secret
    let context = &["for client".as_bytes()];
    let client_secret_okm = initial_secret.expand(context, HKDF_SHA256)
                                          .expect("Could not generate client secret");

    let mut client_secret: Vec<u8> = Vec::with_capacity(SHA256_OUTPUT_LEN);
    client_secret_okm.fill(&mut client_secret)
                     .expect("Could not copy client secret to buffer");

    // Server secret
    let context = &["for server".as_bytes()];
    let server_secret_okm = initial_secret.expand(context, HKDF_SHA256)
                                          .expect("Could not generate server secret");

    let mut server_secret: Vec<u8> = Vec::with_capacity(SHA256_OUTPUT_LEN);
    server_secret_okm.fill(&mut server_secret)
                     .expect("Could not copy server secret to buffer");




    // TODO: Send UDP Datagram with QUIC hello frame
    let udp_socket = UdpSocket::bind("localhost:23756");

    // TODO: Receive datagram with QUIC ServerHello frame



}
