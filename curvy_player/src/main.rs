use raylib::core::audio::RaylibAudio;
use raylib::ffi::IsAudioStreamProcessed;
use raylib::ffi::UpdateAudioStream;

use curvy_core::*;
use curvy_wav::WavStream;

const BUFFER_SIZE: i32 = 4410;

fn main() {
    let mut wav = WavStream::from_file("curvy_wav/samples/Alesis-Fusion-Shakuhachi-C5.wav").unwrap();

    let ray_audio = RaylibAudio::init_audio_device().unwrap();
    ray_audio.set_audio_stream_buffer_size_default(BUFFER_SIZE);
    let stream = ray_audio.new_audio_stream(
        wav.sample_rate(), 
        wav.sample_size(), 
        wav.num_chs() as u32
    );
    stream.play();
    
    let stream = unsafe { stream.inner() };
    let mut buf: Vec<i16> = vec![0; BUFFER_SIZE as usize * wav.num_chs() as usize];

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
                let frame = match wav.frame() {
                    Some(v) => v,
                    None => {
                        finished = true;
                        wav.zero()
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