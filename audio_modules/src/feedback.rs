//! # Feedback
//!
//! Feedback comb filter with variable delay time and gain.
//!
//! Sources to connect: input to delay, delay time, gain.
use crate::delay::Delay;
use audio_graph::{Frame, Module, Sample};

pub struct Feedback {
    delay: Delay,
    delay_input: Vec<Sample>,
    output: Vec<Sample>,
}

impl Feedback {
    pub fn new(channels: u8, sample_rate: u32, max_delay: f64) -> Self {
        let delay = Delay::new(channels, sample_rate, max_delay);
        Feedback {
            delay,
            delay_input: vec![0.0; 2 * channels as usize],
            output: vec![0.0; channels as _],
        }
    }
}

impl Module for Feedback {
    fn inputs(&self) -> u8 {
        3
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (delay_input, output, input) in izip!(
            self.delay_input.chunks_mut(2),
            &self.output,
            input.chunks(3)
        ) {
            // feed output back to the delay module
            delay_input[0] = *output;
            // just copy delay time
            delay_input[1] = input[1];
        }

        self.delay.sample(&self.delay_input);

        for (output, input, delayed) in
            izip!(self.output.iter_mut(), input.chunks(3), self.delay.output())
        {
            let x = input[0];
            let gain = input[2];
            *output = x + gain * delayed;
        }
    }
}
