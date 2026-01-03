# speedy
A Rust implementation of the QUIC protocol.

# Anatomy of QUIC
QUIC is a transport-layer protocol that runs over UDP.

A QUIC connection consists of one or more streams over its duration.

A QUIC stream can be uni- or bidirectional and facilitates the
transmission of related Packets (contained within UDP datagrams).
It is this stream multiplexing functionality that enables QUIC to avoid
head-of-line blocking (one of the weaknesses of TCP).

A key point to note is that multiple Packets can be coalesced into the same
UDP datagram (RFC 9000, Section 12.2).

Each packet contains one or more Frames.