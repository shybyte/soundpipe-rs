extern crate anyhow;
extern crate cpal;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SupportedStreamConfig};

use soundpipe::Soundpipe;
use soundpipe::SoundpipeFactory;

fn main() -> Result<(), anyhow::Error> {
    let host = cpal::default_host();
    let device: Device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config: SupportedStreamConfig = device.default_output_config()?;

    eprintln!("config.s = {:?}", config.sample_format());

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into())?,
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into())?,
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into())?,
    }

    Ok(())
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    eprintln!("sample_rate = {:?}", sample_rate);
    eprintln!("channels = {:?}", channels);

    let sp = Soundpipe::new(sample_rate as i32);
    let bl_saw = sp.bl_saw();
    let bl_saw2 = sp.bl_saw();

    bl_saw.set_freq(220.0);
    bl_saw2.set_freq(110.0);

    let mut next_value = move || (bl_saw.compute() + bl_saw2.compute()) / 2.0;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
