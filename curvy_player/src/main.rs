use std::env;
use std::io::Read;

use raylib::core::audio::RaylibAudio;
use raylib::ffi::IsAudioStreamProcessed;
use raylib::ffi::UpdateAudioStream;

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
    let mut args = env::args();
    args.next();
    let path_to_file = match args.next() {
        Some(path)  => path,
        None        => {
            println!("Please enter a file to listen to");
            return;
        }
    };


    let file_extension: String = path_to_file
        .chars()
        .skip_while(|c| *c != '.')
        //.take_while(|c| *c != '.')
        .collect();
   
    let mut audio_format = match &file_extension[..] {
        ".wav" => AudioFormat::Wav(
            WavStream::from_file(&path_to_file).expect("Wrong wav format")
        ),
        _ => {
            println!("Unknown file format {}", file_extension);
            return;
        }
    };

    let ray_audio = RaylibAudio::init_audio_device().unwrap();
    ray_audio.set_audio_stream_buffer_size_default(BUFFER_SIZE);
    let stream = ray_audio.new_audio_stream(
        audio_format.sample_rate(), 
        audio_format.sample_size(), 
        audio_format.num_chs() as u32
    );
    stream.play();
    
    let stream = unsafe { stream.inner() };
    let mut buf: Vec<i16> = vec![0; BUFFER_SIZE as usize * audio_format.num_chs() as usize];

    let mut finished = false;
    loop {
        let is_processed = unsafe {
            IsAudioStreamProcessed(stream)
        };
        if is_processed {
            if finished {
                break;
            }

            let mut i = 0;
            while i < buf.len() {
                let frame = match audio_format.frame() {
                    Some(v) => v,
                    None => {
                        finished = true;
                        audio_format.zeroed_frame()
                    }
                };

                for sample in frame.samples() {
                    if let AudioSample::PCM16(val) = sample {
                        buf[i] = *val;
                    }

                    i += 1;
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