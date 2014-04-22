#![crate_id = "ssdp"]
#![deny(missing_doc)]

//! Implementation of the SSDP protocol in Rust

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::net::udp::UdpSocket;
use std::task::spawn;
use std::str;


static SSDP_MULTICAST_ADDR : SocketAddr = SocketAddr{ip: Ipv4Addr(239, 255, 255, 250), port: 1900};
static LOCAL_ADDR : SocketAddr = SocketAddr{ip: Ipv4Addr(0, 0, 0, 0), port: 8575};

/// Container for sockets
pub struct SSDPListener {
    local: UdpSocket,
    multicast: UdpSocket,
}

/// Listener for SSDP announcements
pub trait Listener {
    /// Creates the SSDPListener and binds sockets
    fn listen() -> Self;
    /// Sends ssdp:discover to the multicast
    fn send_discover(&mut self);
}

impl Listener for SSDPListener {

    fn listen() -> SSDPListener {
        let listener = SSDPListener {
            local: UdpSocket::bind(LOCAL_ADDR).unwrap(),
            multicast: UdpSocket::bind(SSDP_MULTICAST_ADDR).unwrap(),
        };

        // TODO: Capture announcements etc here
        //       Currently this just listens for unicast messages
        let sock = listener.local.clone();
        spawn(proc() {
            let mut sock2 = sock;
            loop {
                let mut buf = [0, ..1024];
                match sock2.recvfrom(buf) {
                    Ok((amt, src)) => {
                        println!("{}", str::from_utf8(buf.mut_slice_to(amt)).unwrap());
                    }
                    Err(e) => println!("couldn't receive a datagram: {}", e)
                    }
            }
        });
        listener
    }

    fn send_discover(&mut self) {
        let mx = 3;
        let discover_message = format!("M-SEARCH * HTTP/1.1\r\nHOST: {host}\r\nST: ssdp:all\r\nMAN: \"ssdp:discover\"\r\nMX: {mx}\r\n", host=SSDP_MULTICAST_ADDR, mx=mx);
        match self.local.sendto(discover_message.into_bytes(), SSDP_MULTICAST_ADDR) {
            Err(e) => fail!("Couldn't send discover"),
            Ok(_)  => println!("Sent discover"),
        };
    }
}


#[test]
fn test_listner() {
    let mut listener : SSDPListener = Listener::listen();
    listener.send_discover();
}
