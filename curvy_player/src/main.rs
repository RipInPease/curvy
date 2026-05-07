use curvy_wav::*;
use curvy_core::{AudioSample, AudioStream as CurvyAudio};

use std::time::{Instant, Duration};
use raylib::ffi::LoadAudioStream;
use raylib::core::audio::{RaylibAudio, AudioStream};

fn main() {
    let mut wav = WavStream::from_file("curvy_wav/samples/Alesis-Fusion-Shakuhachi-C5.wav").unwrap();
    wav.play();
    let mut samples = Vec::new();

    while let Some(sample) = wav.sample() {
        if let AudioSample::PCM16(int) = sample {
            samples.push(int)
        }
    }

    let audio_device = RaylibAudio::init_audio_device().unwrap();
    let mut audio_stream = audio_device.new_audio_stream(wav.sample_rate(), wav.sample_size(), 1);
    audio_stream.play();
    audio_stream.update(&samples);
    let mut cursor = 0;
    let chunk_size = 1024;
    loop {
        if audio_stream.is_processed() {
            let end = (cursor+chunk_size).min(samples.len());
            audio_stream.update(&samples[cursor..end]);
            cursor += chunk_size;
        }
    }
}