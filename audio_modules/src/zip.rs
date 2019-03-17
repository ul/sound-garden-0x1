//! # Zip
//!
//! Zip the first channel of each input into multi-channel output.
//!
//! Sources to connect: number of inputs equal to channels count.
use audio_graph::{Frame, Module, Sample};

pub struct Zip {
    channels: u8,
    output: Vec<Sample>,
}

impl Zip {
    pub fn new(channels: u8) -> Self {
        Zip {
            channels,
            output: vec![0.0; channels as _],
        }
    }
}

impl Module for Zip {
    fn inputs(&self) -> u8 {
        self.channels
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        self.output.copy_from_slice(&input[..self.channels as _]);
    }
}
