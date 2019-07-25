use crate::util;

use cpal::traits::EventLoopTrait;
use cpal::{StreamData, UnknownTypeOutputBuffer};
use minimp3::Decoder;

use std::fs::File;
use std::path::PathBuf;

pub fn local_file(file: PathBuf) -> Result<(), String> {
    let event_loop = util::get_output_event_loop(cpal::default_host())?;
    let mut local_file = Decoder::new(File::open(file).map_err(|e| e.to_string())?);
    event_loop.run(move |stream_id, stream_result| {
        let sample = match local_file.next_frame() {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error reading: {:?}", err);
                std::process::exit(1);
            }
        };
        let stream_data = match stream_result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
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
                for (buf, byte) in buffer.iter_mut().zip(sample.data) {
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
