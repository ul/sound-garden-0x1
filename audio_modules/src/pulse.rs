//! # Pulse wave
//!
//! Sources to connect: frequency, duty cycle.
use crate::function::Fn2;
use crate::phasor::Phasor;
use crate::pure::rectangle;
use audio_graph::{Frame, Module, Sample};

pub struct Pulse {
    input: Vec<Sample>,
    phasor: Phasor,
    osc: Fn2,
}

impl Pulse {
    pub fn new(channels: u8, sample_rate: u32) -> Self {
        let phasor = Phasor::new(channels, sample_rate);
        let osc = Fn2::new(channels, rectangle);
        let input = vec![0.0; 2 * channels as usize];
        Pulse { input, phasor, osc }
    }
}

impl Module for Pulse {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        self.osc.output()
    }

    fn sample(&mut self, input: &Frame) {
        self.phasor.sample(input);
        for (osc_input, phasor_output, input) in izip!(
            self.input.chunks_mut(2),
            self.phasor.output(),
            input.chunks(2)
        ) {
            osc_input[0] = *phasor_output; // phase
            osc_input[1] = input[1]; // width
        }
        self.osc.sample(&self.input);
    }
}
