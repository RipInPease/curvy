use std::time::Duration;

pub mod utils;

pub trait AudioStream {
    fn update(&mut self);

    fn play(&mut self);
    fn pause(&mut self);
    fn is_playing(&self);

    fn ffw(&mut self, time: Duration);
    fn rew(&mut self, time: Duration);
    fn set_playback_rate(&mut self, rate: f64);
    fn playback_rate(&self);
}


#[derive(Debug)]
pub enum Sample {
    PCM(u32),
    IEEE(f64),
}