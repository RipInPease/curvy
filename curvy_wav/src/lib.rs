use std::io::{self, BufRead, BufReader, Read};
use std::fs::File;
use std::num::NonZero;
use std::path::Path;

use curvy_core::{AudioStream, AudioSample, AudioFrame};
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
        if bits_sample % 8 != 0 {
            let kind = io::ErrorKind::InvalidData;
            return Err(io::Error::new(kind, "Unsupported format. Very fucked up bits per sample"));
        }

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


impl<R: Read> WavStream<R> {
    /// Returns an AudioSample with the value of zero
    pub fn zero(&self) -> AudioFrame {
        let sample = match self.audio_format {
            AudioFormat::PCM => AudioSample::PCM16(0),
            AudioFormat::IEEE => AudioSample::IEEE32(0.0),
        };

        let samples = vec![sample; self.num_chs() as usize].into_boxed_slice();
        AudioFrame::new(samples)
    }


    /// Reads an audio frame from a slice of bytes
    fn get_frame(&self, data: &[u8]) -> AudioFrame {
        let num_samples = ((self.bits_sample / 8) * self.nbr_ch) as usize;
        let mut samples: Vec<AudioSample> = Vec::with_capacity(num_samples as usize);

        for i in 0..num_samples {
            match self.audio_format {
                AudioFormat::IEEE => {
                    let sample_val = utils::f32_from_le_slice(data, i * size_of::<f32>());
                    samples.push(AudioSample::IEEE32(sample_val));
                },
                AudioFormat::PCM => {
                    let sample_val = utils::i16_from_le_slice(data, i * size_of::<i16>());
                    samples.push(AudioSample::PCM16(sample_val));
                }
            }
        }

        AudioFrame::new(samples.into_boxed_slice())
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
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn sample_size(&self) -> u32 {
        self.bits_sample as u32
    }

    fn num_chs(&self) -> u8 {
        self.nbr_ch as u8
    }

    fn frame(&mut self) -> Option<AudioFrame> {
        if !self.is_playing {
            return Some(self.zero())
        }

        // Start of a new data chunk
        if self.bytes_left_chunk == 0 {
            let mut buf = [0; 8];
            if self.source.read_exact(&mut buf).is_err() {
                return None
            }

            if &buf[0..4] != b"data" { return None }
            self.bytes_left_chunk = utils::u32_from_le_slice(&buf,4);
        }

        // Read the next frame
        let mut buf = vec![0; self.bytes_block as usize];
        if self.source.read_exact(&mut buf).is_err() {
            return None
        }
        self.bytes_left_chunk -= self.bytes_block as u32;
        let frame = self.get_frame(&buf);

        Some(frame)
    }
}