use std::f64::consts::TAU;
use raylib::core::audio::RaylibAudio;
use raylib::ffi::IsAudioStreamProcessed;
use raylib::ffi::UpdateAudioStream;

use curvy_core::*;
use curvy_wav::WavStream;

const FREQ: f64 = 440.0;
const SAMPLE_RATE: u32 = 44100;
const BUFFER_SIZE: i32 = 4410;
const AMP: f64 = 0.2; // volume (0.0–1.0)

fn main() {
    let mut wav = WavStream::from_file("curvy_wav/samples/Alesis-Fusion-Shakuhachi-C5.wav").unwrap();

    let ray_audio = RaylibAudio::init_audio_device().unwrap();
    ray_audio.set_audio_stream_buffer_size_default(BUFFER_SIZE);
    let stream = ray_audio.new_audio_stream(
        wav.sample_rate(), 
        wav.sample_size(), 
        2
    );
    stream.play();
    
    let stream = unsafe { stream.inner() };

    let mut theta = 0.0;
    let step = TAU * FREQ / SAMPLE_RATE as f64;
    let mut buf: Vec<i16> = vec![0; BUFFER_SIZE as usize];

    loop {
        let is_processed = unsafe {
            IsAudioStreamProcessed(stream)
        };
        if is_processed {
            for s in &mut buf {
                let frame = wav.frame().unwrap_or(wav.zero());
                for sample in frame.samples() {
                    if let AudioSample::PCM16(val) = sample {
                        //println!("{val}");
                        *s = *val;
                    }
                }
            }

            unsafe {
                UpdateAudioStream(
                    stream, 
                    buf.as_ptr() as *const std::os::raw::c_void, 
                    BUFFER_SIZE
                );
            }
        }
    }
}