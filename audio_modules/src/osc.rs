//! # Oscillator
//!
//! Sources to connect: frequency.
use crate::function::Fn1;
use crate::phasor::{Phasor, Phasor0};
use audio_graph::{Frame, Module, Sample};

pub struct Osc {
    phasor: Phasor,
    osc: Fn1,
}

impl Osc {
    pub fn new(channels: u8, sample_rate: u32, f: fn(Sample) -> Sample) -> Self {
        let phasor = Phasor::new(channels, sample_rate);
        let osc = Fn1::new(channels, f);
        Osc { phasor, osc }
    }
}

impl Module for Osc {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        self.osc.output()
    }

    fn sample(&mut self, input: &Frame) {
        self.phasor.sample(input);
        self.osc.sample(self.phasor.output());
    }
}

pub struct OscPhase {
    phasor: Phasor0,
    osc: Fn1,
}

impl OscPhase {
    pub fn new(channels: u8, sample_rate: u32, f: fn(Sample) -> Sample) -> Self {
        let phasor = Phasor0::new(channels, sample_rate);
        let osc = Fn1::new(channels, f);
        OscPhase { phasor, osc }
    }
}

impl Module for OscPhase {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        self.osc.output()
    }

    fn sample(&mut self, input: &Frame) {
        self.phasor.sample(input);
        self.osc.sample(self.phasor.output());
    }
}
