use cpal::{
    traits::{DeviceTrait, EventLoopTrait, HostTrait},
    SampleFormat,
};

fn get_device_and_format<H: HostTrait>(host: &H) -> Result<(H::Device, cpal::Format), String> {
    let device = host
        .default_output_device()
        .ok_or("No output devices available.")?;
    let format = device
        .supported_output_formats()
        .map_err(|_| "error while querying formats")?
        .find(|format| format.data_type == SampleFormat::I16)
        .ok_or("no supported format?!")?
        .with_max_sample_rate();
    Ok((device, format))
}

pub fn get_output_event_loop<H: HostTrait>(host: H) -> Result<H::EventLoop, String> {
    let (device, format) = get_device_and_format(&host)?;
    let event_loop = host.event_loop();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop
        .play_stream(stream_id)
        .map_err(|_| "failed to play_stream")?;
    Ok(event_loop)
}

pub fn get_input_event_loop<H: HostTrait>(host: H) -> Result<H::EventLoop, String> {
    let (device, format) = get_device_and_format(&host)?;
    let event_loop = host.event_loop();
    let stream_id = event_loop.build_input_stream(&device, &format).unwrap();
    event_loop
        .play_stream(stream_id)
        .map_err(|_| "failed to play_stream")?;
    Ok(event_loop)
}

