#![crate_id = "ssdp"]
#![deny(missing_doc)]

//! Implementation of the SSDP protocol in Rust

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::net::udp::UdpSocket;
use std::str;


static SSDP_MULTICAST_ADDR : SocketAddr = SocketAddr{ip: Ipv4Addr(239, 255, 255, 250), port: 1900};

fn open_socket() -> UdpSocket {
    match UdpSocket::bind(SSDP_MULTICAST_ADDR) {
        Ok(s) => s,
        Err(e) => fail!("couldn't bind socket: {}", e),
    }
}

fn send_discover_all() {
    let discover_message = format!(
"M-SEARCH * HTTP/1.1
HOST: {host}
ST: ssdp:all
MAN: \"ssdp:discover\"
MX: {mx}
", host=SSDP_MULTICAST_ADDR, mx=3);
    let mut sock = open_socket();
    match sock.sendto(discover_message.into_bytes(), SSDP_MULTICAST_ADDR) {
        Err(e) => fail!("Couldn't send discover"),
        Ok(_)  => println!("Sent discover"),
    };
    /* TODO listen for responses */
    drop(sock);
}

#[test]
fn test_discover_all() {
    send_discover_all();
}

#[test]
fn test_open_socket() {
    let mut sock = open_socket();
    assert!(sock.socket_name().unwrap().port == 1900);
    drop(sock);
}
