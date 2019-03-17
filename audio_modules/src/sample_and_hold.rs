//! Sample & Hold
//!
//! Sources to connect: trigger, input.
use audio_graph::{Frame, Module, Sample};

pub struct SampleAndHold {
    output: Vec<Sample>,
}

impl SampleAndHold {
    pub fn new(channels: u8) -> Self {
        SampleAndHold {
            output: vec![0.0; channels as _],
        }
    }
}

impl Module for SampleAndHold {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (output, input) in self.output.iter_mut().zip(input.chunks(2)) {
            let t = input[0];
            let x = input[1];
            *output = *output * (1.0 - t) + x * t
        }
    }
}
