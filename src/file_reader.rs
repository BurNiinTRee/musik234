use discord::voice::AudioSource;

use std::fs::File;
use std::i16;
use error::*;


pub struct Audio;


impl Audio {
    pub fn new() -> Audio {
        Audio
    }
}


impl AudioSource for Audio {
    fn is_stereo(&mut self) -> bool {
        false
    }

    fn read_frame(&mut self, buffer: &mut [i16]) -> Option<usize> {
        for (i, mut byte) in buffer.iter_mut().enumerate() {
            *byte = (i as i16 % 2) * i16::MAX;
        }
        println!("{:?}", buffer);
        Some(buffer.len())
    }
}
