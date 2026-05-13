pub mod utils;

pub trait AudioStream {
    fn sample_rate(&self) -> u32;
    fn sample_size(&self) -> u32;
    fn num_chs(&self) -> u8;

    /// Return None if there is no more audio
    fn frame(&mut self) -> Option<AudioFrame>;
}


#[derive(Debug, Copy, Clone)]
pub enum AudioSample {
    PCM8(u8),
    PCM16(i16),
    IEEE32(f32)
}


/// Contains one sample for every channel
#[derive(Debug, Clone)]
pub struct AudioFrame {
    samples: Box<[AudioSample]>
}


impl AudioFrame {
    pub fn new(samples: Box<[AudioSample]>) -> Self {
        Self {
            samples
        }
    }


    /// An Iterator over all [`AudioSample`]
    pub fn samples(&self) -> Samples<'_> {
        Samples { 
            frame: self, 
            i: 0, max_i: self.samples.len() 
        }
    }
}


/// An iterator over all samples in an [`AudioFrame`]
pub struct Samples<'a> {
    frame: &'a AudioFrame,
    i: usize,
    max_i: usize
}


impl<'a> Iterator for Samples<'a> {
    type Item = &'a AudioSample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.max_i {
            None
        } else {
            let res = &self.frame.samples[self.i];
            self.i += 1;
            Some(res)
        }
    }
}