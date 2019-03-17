//! Input
//!
//! Forward external input to output.
//!
//! Sources to connect: none required.

use audio_graph::{Frame, Module, Sample};

pub struct Input {
    output: Vec<Sample>,
}

impl Input {
    pub fn new(channels: u8) -> Self {
        Input {
            output: vec![0.0; channels as _],
        }
    }
}

impl Module for Input {
    fn inputs(&self) -> u8 {
        0
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let channels = self.output.len();
        self.output.clone_from_slice(&input[..channels]);
    }
}
