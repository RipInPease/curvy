use std::time::Duration;

pub mod utils;

pub trait AudioStream {
    fn update(&mut self);

    fn play(&mut self);
    fn pause(&mut self);
    fn is_playing(&self) -> bool;

    fn ffw(&mut self, time: Duration);
    fn rew(&mut self, time: Duration);
    fn set_playback_rate(&mut self, rate: f64);
    fn playback_rate(&self) -> f64;

    fn sample_rate(&self) -> u32;
    fn sample_size(&self) -> u32;

    /// Return None if there is no more audio, or the stream is paused
    fn sample(&mut self) -> Option<AudioSample>;
}


#[derive(Debug)]
pub enum AudioSample {
    PCM8(u8),
    PCM16(i16),
    IEEE32(f32)
}