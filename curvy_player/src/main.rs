use curvy_wav::*;
use curvy_core::{AudioSample, AudioStream as CurvyAudio};

use std::time::{Instant, Duration};
use raylib::ffi::LoadAudioStream;
use raylib::core::audio::{RaylibAudio, AudioStream};

const BUFFER_SIZE: i32 = 800;

fn main() {
    let mut wav = WavStream::from_file("curvy_wav/samples/Alesis-Fusion-Shakuhachi-C5.wav").unwrap();
    wav.play();


    let audio_device = RaylibAudio::init_audio_device().unwrap();
    audio_device.set_audio_stream_buffer_size_default(BUFFER_SIZE);

    let mut audio_stream = audio_device.new_audio_stream(
        wav.sample_rate(), 
        wav.sample_size(), 
        1
    );
    //println!("Sample size: {}", wav.sample_size());
    //panic!();
    
    audio_stream.play();
    let mut buf = vec![0; BUFFER_SIZE as usize];
    audio_stream.update(&buf);
    loop {
        //println!("Here");
        if audio_stream.is_processed() {
            for i in 0..BUFFER_SIZE {
                if let Some(sample) = wav.sample() {
                    if let AudioSample::PCM16(val) = sample {
                        buf[i as usize] = val;
                    }
                }
            }

            audio_stream.update(&buf);
        }
    }
}