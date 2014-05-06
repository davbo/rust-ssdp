#![crate_id = "ssdp"]

//! Implementation of the SSDP protocol in Rust

extern crate native;
extern crate http;

use http::memstream::MemReaderFakeStream;
use http::buffer::BufferedStream;
use http::server::request::Request;

use native::io::net::UdpSocket;

use std::io::MemWriter;
use std::io::net::udp::UdpStream;
use std::io::net::ip::{SocketAddr, IpAddr, Ipv4Addr};
use std::rt::rtio::RtioUdpSocket;
use std::task::spawn;
use std::str;


static SSDP_MULTICAST_IPADDR : IpAddr = Ipv4Addr(239, 255, 255, 250);
static SSDP_MULTICAST_SOCKET : SocketAddr = SocketAddr{ip: SSDP_MULTICAST_IPADDR, port: 1900};


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
                            let mut stream = BufferedStream::new(
                                MemReaderFakeStream::new(Vec::from_slice(buf.slice_to(amt))));
                            /*let mut b = [0u8, ..4096];*/
                            /*let len = stream.read(b);*/
                            /*println!("{}", str::from_utf8(b.slice_to(len.unwrap())));*/
                            let (request, err_status) = Request::load(&mut stream);
                            println!("{} - {}", request.method, request.request_uri);
                            let mut w = MemWriter::new();
                            request.headers.write_all(&mut w);
                            println!("{}", str::from_utf8(w.unwrap().as_slice()));
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
