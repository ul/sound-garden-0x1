//! # Stereo panner
//!
//! Sources to connect: left, right, position.
use crate::pure;
use audio_graph::{Frame, Module, Sample};

pub struct Pan1 {
    output: Vec<Sample>,
}

impl Pan1 {
    pub fn new(channels: u8) -> Self {
        Pan1 {
            output: vec![0.0; channels as _],
        }
    }
}

impl Module for Pan1 {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let (l, r) = pure::pan(input[0], input[1], input[2]);
        self.output[0] = l;
        self.output[1] = r;
    }
}

pub struct Pan2 {
    output: Vec<Sample>,
}

impl Pan2 {
    pub fn new(channels: u8) -> Self {
        Pan2 {
            output: vec![0.0; channels as _],
        }
    }
}

impl Module for Pan2 {
    fn inputs(&self) -> u8 {
        3
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let l = input[0]; // left of the first input
        let r = input[1]; // left of the second input
        let c = input[2]; // left of the position
        let (l, r) = pure::pan(l, r, c);
        self.output[0] = l;
        self.output[1] = r;
    }
}

pub struct Pan3 {
    output: Vec<Sample>,
}

impl Pan3 {
    pub fn new(channels: u8) -> Self {
        Pan3 {
            output: vec![0.0; channels as _],
        }
    }
}

impl Module for Pan3 {
    fn inputs(&self) -> u8 {
        3
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (channel, (output, input)) in self.output.iter_mut().zip(input.chunks(3)).enumerate() {
            let l = input[0];
            let r = input[1];
            let c = input[2];
            // Left output is left of pan of left inputs, right output right is pan of right inputs.
            // I don't know who need this.
            *output = match channel {
                0 => 1.0_f64.min(1.0 - c).sqrt() * l + 0.0_f64.max(-c).sqrt() * r,
                1 => 0.0_f64.max(c).sqrt() * l + 1.0_f64.min(1.0 + c).sqrt() * r,
                _ => 0.0,
            }
        }
    }
}
