use std::io::{self, BufRead, BufReader, Read};
use std::fs::File;
use std::path::Path;

use curvy_core::{AudioStream, AudioSample};
use curvy_core::utils;


#[derive(Debug)]
pub struct WavStream<R: Read> {
    source: R,
    is_playing: bool,
    playback_rate: f64,

    // Data format
    //block_size: u32,
    audio_format: AudioFormat,
    nbr_ch: u16,
    sample_rate: u32,
    bytes_sec: u32,
    bytes_block: u16,
    bits_sample: u16,

    // The amount of bytes left in the current block of data
    bytes_left_chunk: u32,
}


impl WavStream<BufReader<File>> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let file = File::options()
            .read(true)
            .open(path)?;

        let file_data = file.metadata()?;
        let mut source = BufReader::new(file);

        // Check the file starts with "RIFF"
        let mut buf = [0; 4];
        source.read_exact(&mut buf)?;
        if &buf != b"RIFF" {
            return Err(utils::invalid_format_error())
        }

        // Check the file sizes corresponds
        source.read_exact(&mut buf)?;
        let file_size = utils::u32_from_le_slice(&buf, 0);
        if file_size as u64 != file_data.len() - 8 {
            return Err(utils::invalid_format_error())
        }

        // Check the file format is "WAVE"
        source.read_exact(&mut buf)?;
        if &buf != b"WAVE" {
            return Err(utils::invalid_format_error())
        }

        // Check the format block ID is "fmt "
        source.read_exact(&mut buf)?;
        if &buf != b"fmt " {
            return Err(utils::invalid_format_error())
        }


        // Read the rest of the data format
        let mut buf = [0; 20];
        source.read_exact(&mut buf)?;

        //let block_size = utils::u32_from_le_slice(&buf, 0);
        let audio_format: AudioFormat = 
            match AudioFormat::try_from(utils::u16_from_le_slice(&buf, 4)) {
                Ok(fmt) => fmt,
                Err(_)  => return Err(utils::invalid_format_error())
            };
        let nbr_ch = utils::u16_from_le_slice(&buf, 6);
        let sample_rate = utils::u32_from_le_slice(&buf, 8);
        let bytes_sec = utils::u32_from_le_slice(&buf, 12);
        let bytes_block = utils::u16_from_le_slice(&buf, 16);
        let bits_sample = utils::u16_from_le_slice(&buf, 18);

        Ok(Self { 
            source,
            is_playing: false,
            playback_rate: 1.0,
            //block_size,
            audio_format,
            nbr_ch,
            sample_rate,
            bytes_sec,
            bytes_block,
            bits_sample,
            bytes_left_chunk: 0
        })
    }
}


#[derive(Debug)]
enum AudioFormat {
    PCM,
    IEEE,
}

impl TryFrom<u16> for AudioFormat {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1   =>  Ok(Self::PCM),
            3   =>  Ok(Self::IEEE),
            _   =>  Err(())
        }
    }
}


impl<R: Read> AudioStream for WavStream<R> {
    fn update(&mut self) {
        
    }

    fn play(&mut self) {
        self.is_playing = true
    }

    fn pause(&mut self) {
        self.is_playing = false
    }

    fn is_playing(&self) -> bool {
        self.is_playing
    }

    fn ffw(&mut self, time: std::time::Duration) {
        
    }

    fn rew(&mut self, time: std::time::Duration) {
        
    }

    fn set_playback_rate(&mut self, rate: f64) {
        self.playback_rate = rate
    }

    fn playback_rate(&self) -> f64 {
        self.playback_rate
    }

    fn sample(&mut self) -> Option<AudioSample> {
        None
    }
}