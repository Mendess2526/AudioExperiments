use crate::sample::Sample;
use crate::util;

use cpal::traits::EventLoopTrait;
use cpal::{StreamData, UnknownTypeOutputBuffer};

use std::net::ToSocketAddrs;
use std::net::UdpSocket;

pub fn server<A: ToSocketAddrs>(addr: A) -> Result<(), String> {
    let event_loop = util::get_output_event_loop(cpal::default_host())?;
    let socket = UdpSocket::bind(addr).map_err(|e| e.to_string())?;
    event_loop.run(move |stream_id, stream_result| {
        let mut buf = [0; 4069 * 4];
        let (amt, _src) = match socket.recv_from(&mut buf) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error receiving: {:?}", err);
                return;
            }
        };
        let stream_data = match stream_result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                return;
            }
        };
        let sample =
            match bincode::deserialize::<Sample<i16>>(&buf[0..amt]).map_err(|e| e.to_string()) {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("Error deserializing: {:?}", err);
                    return;
                }
            };
        match stream_data {
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::U16(_),
            } => (),
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::I16(mut buffer),
            } => {
                for (buf, byte) in buffer.iter_mut().zip(sample) {
                    *buf = byte;
                }
            }
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::F32(_),
            } => (),
            _ => panic!("Wtf is going on!"),
        }
    })
}
