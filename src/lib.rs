#![crate_id = "ssdp"]

//! Implementation of the SSDP protocol in Rust

extern crate native;

use std::io::net::ip::{SocketAddr, IpAddr, Ipv4Addr};
use native::io::net::UdpSocket;
use std::rt::rtio::RtioUdpSocket;
use std::task::spawn;
use std::str;


static SSDP_MULTICAST_IPADDR : IpAddr = Ipv4Addr(239, 255, 255, 250);
pub static SSDP_MULTICAST_SOCKET : SocketAddr = SocketAddr{ip: SSDP_MULTICAST_IPADDR, port: 1900};


/// Listener for SSDP announcements
pub trait SSDPListener {
    /// Start listening on the multicast
    fn listen(&mut self);
    /// Sends ssdp:discover to the multicast
    fn send_discover(&mut self);
}

impl SSDPListener for UdpSocket {

    fn listen(&mut self) {
        self.join_multicast(SSDP_MULTICAST_IPADDR).unwrap();
        self.multicast_time_to_live(2).unwrap();
        let sock = self.clone();
        spawn(proc() {
            let mut sock2 = sock;
            loop {
                let mut buf = [0, ..1024];
                match sock2.recvfrom(buf) {
                    Ok((amt, src)) => {
                        println!("{}\r\n{}", src, str::from_utf8(buf.mut_slice_to(amt)).unwrap());
                    }
                    Err(e) => println!("couldn't receive a datagram {}", e)
                    }
            }
        });
    }

    fn send_discover(&mut self) {
        let mx = 3;
        let discover_message = format!("M-SEARCH * HTTP/1.1\r\nHOST: {host}\r\nST: ssdp:all\r\nMAN: \"ssdp:discover\"\r\nMX: {mx}\r\n", host=SSDP_MULTICAST_SOCKET, mx=mx);
        match self.sendto(discover_message.into_bytes(), SSDP_MULTICAST_SOCKET) {
            Err(e) => fail!("Couldn't send discover {}", e),
            Ok(_)  => println!("Sent discover"),
        };
    }
}


#[test]
fn test_listner() {
    let mut listener : UdpSocket = UdpSocket::bind(SSDP_MULTICAST_SOCKET).unwrap();
    listener.send_discover();
}
