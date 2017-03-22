use chrono::prelude::*;
use util::slice_util;

pub mod dump;

pub struct Dumper {
    pub dumped_packets: Vec<dump::UdpDump>,
    prev_dumped_time: Option<DateTime<UTC>>
}

impl Dumper {
    pub fn new() -> Dumper {
        Dumper {
            prev_dumped_time: None,
            dumped_packets: vec!(),
        }
    }

    pub fn dumps(&self) -> Vec<dump::UdpDump> {
        let c = self.dumped_packets.to_vec();
        c
    }

    pub fn dump(&mut self, packet: &[u8]) {
        let mut delta: i64 = 0;

        let utc_now: DateTime<UTC> = UTC::now();

        match self.prev_dumped_time {
            Some(prev_time) => {
                delta = utc_now.timestamp() - prev_time.timestamp();
            },
            None => ()
        }

        let dump = dump::UdpDump::new(packet, utc_now, delta);
        self.dumped_packets.push(dump);
    }
}
