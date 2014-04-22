extern crate ssdp;
use ssdp::{Listener, SSDPListener};

fn main() {
    let mut listener : SSDPListener = Listener::listen();
    listener.send_discover();
}
