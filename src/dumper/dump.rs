use chrono::prelude::*;
use std::convert::AsMut;
use util::slice_util::clone_into_array;

#[derive(Clone)]
pub struct UdpDump {
    pub payload: Vec<u8>,
    pub arrival_time: DateTime<UTC>,
    pub delta_time_from_prev: i64
}


impl UdpDump {
    pub fn new(packet: &[u8], arrival_time: DateTime<UTC>, delta_time_from_prev: i64) -> UdpDump {
        UdpDump {
            payload: packet.to_vec(),
            arrival_time: arrival_time,
            delta_time_from_prev: delta_time_from_prev,
        }
    }
}
