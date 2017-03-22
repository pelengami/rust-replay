extern crate pnet;
extern crate chrono;

use pnet::datalink::{self, NetworkInterface};

mod dumper;
mod listener;

mod util {
    pub mod slice_util;
}

fn main() {
    let interfaces = datalink::interfaces();
    let ref interface = interfaces[1];

    let mut listener = listener::Listener::new();
    listener.run(&interface);
}
