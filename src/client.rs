use crate::sample::Sample;
use crate::util;

use cpal::traits::EventLoopTrait;
use cpal::{StreamData, UnknownTypeInputBuffer};
use std::net::ToSocketAddrs;
use std::net::UdpSocket;

pub fn client<A: ToSocketAddrs + Send>(addr: A, server: A) -> Result<(), String> {
    let event_loop = util::get_input_event_loop(cpal::default_host())?;
    let socket = UdpSocket::bind(addr).map_err(|e| e.to_string())?;
    event_loop.run(move |stream_id, stream_result| {
        let stream_data = match stream_result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                return;
            }
        };
        let data: Vec<i16> = match stream_data {
            StreamData::Input {
                buffer: UnknownTypeInputBuffer::U16(_),
            } => panic!("u16"),
            StreamData::Input {
                buffer: UnknownTypeInputBuffer::I16(buffer),
            } => buffer.as_ref().into(),
            StreamData::Input {
                buffer: UnknownTypeInputBuffer::F32(_),
            } => panic!("F32"),
            _ => panic!("Wtf is going on!"),
        };
        let sample = match bincode::serialize(&Sample::from(data)) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error serializing: {:?}", err);
                return;
            }
        };
        if let Err(err) = socket.send_to(&sample, &server) {
            eprintln!("Error sending: {:?}", err)
        };
    })
}
