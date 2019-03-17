//! # Phasor
//!
//! ```
//!  1     /|    /|    /|    /|
//!       / |   / |   / |   / |
//!  0   /  |  /  |  /  |  /  |
//!     /   | /   | /   | /   |
//! -1 /    |/    |/    |/    |
//! ```
//!
//! Phasor module generates a saw wave in the range -1..1.
//! Frequency is controlled by the input for each channel separately and can be variable.
//!
//! It is called phasor because it could be used as input phase for other oscillators, which become
//! just pure transformations then and are not required to care about handling varying frequency by
//! themselves anymore.
//!
//! Sources to connect: frequency.
use audio_graph::{Frame, Module, Sample};

pub struct Phasor {
    phases: Vec<Sample>,
    sample_period: Sample,
}

impl Phasor {
    pub fn new(channels: u8, sample_rate: u32) -> Self {
        Phasor {
            phases: vec![0.0; channels as _],
            sample_period: Sample::from(sample_rate).recip(),
        }
    }
}

impl Module for Phasor {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.phases
    }

    fn sample(&mut self, input: &Frame) {
        for (phase, frequency) in self.phases.iter_mut().zip(input) {
            let dx = frequency * self.sample_period;
            *phase = ((*phase + dx + 1.0) % 2.0) - 1.0;;
        }
    }
}

pub struct Phasor0 {
    phases: Vec<Sample>,
    sample_period: Sample,
}

impl Phasor0 {
    pub fn new(channels: u8, sample_rate: u32) -> Self {
        Phasor0 {
            phases: vec![0.0; channels as _],
            sample_period: Sample::from(sample_rate).recip(),
        }
    }
}

impl Module for Phasor0 {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.phases
    }

    fn sample(&mut self, input: &Frame) {
        for (phase, input) in self.phases.iter_mut().zip(input.chunks(2)) {
            let frequency = input[0];
            let phase0 = input[1];
            let dx = frequency * self.sample_period;
            *phase = ((*phase + phase0 + dx + 1.0) % 2.0) - 1.0;
        }
    }
}
