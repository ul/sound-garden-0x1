//! # Noise
//!
//! White noise.
//!
//! Sources to connect: none required.
use audio_graph::{Frame, Module, Sample};
use rand::{rngs::SmallRng, Rng, SeedableRng};

pub struct WhiteNoise {
    rng: SmallRng,
    output: Vec<Sample>,
}

impl WhiteNoise {
    pub fn new(channels: u8) -> Self {
        WhiteNoise {
            rng: SmallRng::from_entropy(),
            output: vec![0.0; channels as _],
        }
    }
}

impl Module for WhiteNoise {
    fn inputs(&self) -> u8 {
        0
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, _input: &Frame) {
        for sample in self.output.iter_mut() {
            *sample = self.rng.gen_range(-1.0, 1.0);
        }
    }
}
