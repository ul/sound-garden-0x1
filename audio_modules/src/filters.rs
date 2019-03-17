//! Filters
//!
//! Basic IIR low/high-pass filters.
//!
//! Sources to connect: input, cut-off frequency.
use audio_graph::{Frame, Module, Sample};

pub struct LPF {
    output: Vec<Sample>,
    sample_angular_period: Sample,
}

impl LPF {
    pub fn new(channels: u8, sample_rate: u32) -> Self {
        let sample_angular_period = 2.0 * std::f64::consts::PI / Sample::from(sample_rate);
        LPF {
            output: vec![0.0; channels as _],
            sample_angular_period,
        }
    }
}

impl Module for LPF {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (output, input) in self.output.iter_mut().zip(input.chunks(2)) {
            let x = input[0];
            let frequency = input[1];
            let k = frequency * self.sample_angular_period;
            let a = k / (k + 1.0);
            *output += a * (x - *output);
        }
    }
}

pub struct HPF {
    output: Vec<Sample>,
    sample_angular_period: Sample,
    x_prime: Vec<Sample>,
}

impl HPF {
    pub fn new(channels: u8, sample_rate: u32) -> Self {
        let sample_angular_period = 2.0 * std::f64::consts::PI / Sample::from(sample_rate);
        HPF {
            output: vec![0.0; channels as _],
            sample_angular_period,
            x_prime: vec![0.0; channels as _],
        }
    }
}

impl Module for HPF {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (output, input, x_prime) in izip!(
            self.output.iter_mut(),
            input.chunks(2),
            self.x_prime.iter_mut()
        ) {
            let x = input[0];
            let frequency = input[1];
            let k = frequency * self.sample_angular_period;
            let a = 1.0 / (k + 1.0);
            *output = a * (*output + x - *x_prime);
            *x_prime = x;
        }
    }
}
