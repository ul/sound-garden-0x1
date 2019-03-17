//! Parameter
//!
//! Extract parameter from external input by index.
//!
//! Sources to connect: none required.

use audio_graph::{Frame, Module, Sample};

pub struct Parameter {
    index: usize,
    output: Vec<Sample>,
}

impl Parameter {
    pub fn new(channels: u8, index: u8) -> Self {
        Parameter {
            index: index as _,
            output: vec![0.0; channels as _],
        }
    }
}

impl Module for Parameter {
    fn inputs(&self) -> u8 {
        0
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let channels = self.output.len();
        let value = input[channels + self.index];
        for output in self.output.iter_mut() {
            *output = value;
        }
    }
}
