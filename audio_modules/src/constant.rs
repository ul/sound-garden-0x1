//! # Constant
//!
//! Constant module always outputs the same given sample in all channels.
//!
//! Sources to connect: none required.
use audio_graph::{Frame, Module, Sample};

pub struct Constant {
    values: Vec<Sample>,
}

impl Constant {
    pub fn new(channels: u8, x: Sample) -> Self {
        Constant {
            values: vec![x; channels as _],
        }
    }
}

impl Module for Constant {
    fn inputs(&self) -> u8 {
        0
    }

    fn output(&self) -> &Frame {
        &self.values
    }

    fn sample(&mut self, _input: &Frame) {}
}
