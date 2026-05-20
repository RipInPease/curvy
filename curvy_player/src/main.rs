use std::io::{Read, BufReader};
use std::fs;

use sdl3::audio::{AudioFormat as SdlAudioFormat, AudioSpec, AudioStream as SdlAudioStream};

use curvy_core::*;
use curvy_wav::WavStream;

const BUFFER_SIZE: i32 = 4410;


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
    let mut audio_format = match open_audio() {
        Some(v) => v,
        None    => return
    };

    
    let sdl_context = sdl3::init().expect("Failed to init SDL3");
    let audio_system = sdl_context.audio().expect("Failed to create audio system using SDL3");
    
}


fn open_audio() -> Option<AudioFormat<BufReader<fs::File>>> {
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