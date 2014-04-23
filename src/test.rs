extern crate ssdp;
extern crate native;

use native::io::net::UdpSocket;
use ssdp::{SSDPListener, SSDP_MULTICAST_SOCKET};

fn main() {
    let mut listener : UdpSocket = UdpSocket::bind(SSDP_MULTICAST_SOCKET).unwrap();
    listener.listen();
}
