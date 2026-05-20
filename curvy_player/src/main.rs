use std::io::{Read, BufReader};
use std::fs;

use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};

use curvy_core::*;
use curvy_wav::WavStream;


enum AudioFormat<R: Read> {
    Wav(WavStream<R>)
}

impl<R: Read> AudioStream for AudioFormat<R> {
    fn frame(&mut self) -> Option<AudioFrame> {
        match self {
            Self::Wav(wav) => wav.frame()
        }
    }

    fn zeroed_frame(&self) -> AudioFrame {
        match self {
            Self::Wav(wav) => wav.zeroed_frame()
        }
    }

    fn num_chs(&self) -> u8 {
        match self {
            Self::Wav(wav) => wav.num_chs()
        }
    }

    fn sample_rate(&self) -> u32 {
        match self {
            Self::Wav(wav) => wav.sample_rate()
        }
    }

    fn sample_size(&self) -> u32 {
        match self {
            Self::Wav(wav) => wav.sample_size()
        }
    }
}


fn main() {
    let mut audio_format = match open_audio_file() {
        Some(v) => v,
        None    => return
    };

    let cpal_host = cpal::default_host();
    let device = cpal_host.default_output_device().expect("Not output device available");

    let config = cpal::StreamConfig {
        channels: audio_format.num_chs() as u16,
        sample_rate: audio_format.sample_rate(),
        buffer_size: cpal::BufferSize::Default,
    };

    let sample_format = match audio_format.sample_size() {
        8   => cpal::SampleFormat::U8,
        16  => cpal::SampleFormat::I16,
        32  => cpal::SampleFormat::F32,
        b   => {
            println!("Unknown sample format: {b} bits per sample");
            return
        }
    };

    let stream = device.build_output_stream(
        &config, 
        move |data: &mut [i16], info: &cpal::OutputCallbackInfo| {
            // react to stream events and read or write stream data here.
            update_audio_buffer(data, info, &mut audio_format);
        },
        move |err| {
            // react to errors here.
        },
        None // None=blocking, Some(Duration)=timeout
    ).expect("Failed to open audio stream");

    stream.play().unwrap();
    loop {}
}


fn open_audio_file() -> Option<AudioFormat<BufReader<fs::File>>> {
    use std::env;
    let mut args = env::args();
    args.next();
    let path_to_file = match args.next() {
        Some(path)  => path,
        None        => {
            println!("Please enter a file to listen to");
            return None;
        }
    };


    let file_extension: String = path_to_file
        .chars()
        .rev()
        .take_while(|b| *b != '.')
        .collect::<String>()
        .chars().rev().collect();
   
    let audio_format = match &file_extension[..] {
        "wav" => AudioFormat::Wav(
            WavStream::from_file(&path_to_file).expect("Wrong wav format")
        ),
        _ => {
            println!("Unknown file format .{}", file_extension);
            return None;
        }
    };

    Some(audio_format)
}


fn update_audio_buffer<S>(data: &mut [i16], _: &cpal::OutputCallbackInfo, audio_stream: &mut S) 
where
    S: AudioStream,
{
    let mut i = 0;
    let i_max = data.len();

    while i < i_max && let Some(frame) = audio_stream.frame() {
        for sample in frame.samples() {
            if let AudioSample::PCM16(val) = sample {
                data[i] = *val;
                //println!("sample: {}", val);
            }
            i += 1;
        }
    }
}