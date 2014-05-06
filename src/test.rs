extern crate ssdp;
extern crate native;

use native::io::net::UdpSocket;
use ssdp::{SSDPListener};

use std::io::net::ip::{SocketAddr, IpAddr, Ipv4Addr};
static LOCAL_SOCK : SocketAddr = SocketAddr{ip: Ipv4Addr(0,0,0,0), port: 1900};

fn main() {
    let mut listener : UdpSocket = UdpSocket::bind(LOCAL_SOCK).unwrap();
    listener.listen();
    listener.send_discover();
}
