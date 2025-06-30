// An implementation of TLS 1.3 for use with QUIC.
// This will contain the logic for key generation, the encryption handshake

// Record






// Handshake

#[repr(C)]
enum HandshakeType
{
    ClientHello = 1,
    ServerHello = 2,
    NewSessionTicket = 4,
    EndOfEarlyData = 5,
    EncryptedExtensions = 8,
    Certificate = 11,
    CertificateRequest = 13,
    CertificateVerify = 15,
    Finished = 20,
    KeyUpdate = 24,
    MessageHash = 254,
}

#[repr(C)]
struct HandshakeHeader
{
    type_and_length: u32,  // First byte is message type
                           // Next 3 bytes are length of ClientHello data
}