//! # Delay
//!
//! Variable signal delay up to maximum period.
//!
//! Sources to connect: input to delay, delay time.
use audio_graph::{Frame, Module, Sample};
use std::collections::VecDeque;

pub struct Delay {
    buffers: Vec<VecDeque<Sample>>,
    mask: usize,
    frame_number: usize,
    sample_rate: Sample,
    output: Vec<Sample>,
}

impl Delay {
    pub fn new(channels: u8, sample_rate: u32, max_delay: f64) -> Self {
        let sample_rate = Sample::from(sample_rate);
        // +1 because interpolation looks for the next sample
        // next_power_of_two to trade memory for speed by replacing `mod` with `&`
        let max_delay_frames = ((sample_rate * max_delay) as usize + 1).next_power_of_two();
        let mask = max_delay_frames - 1;
        let mut buffers = Vec::with_capacity(channels as _);
        for _ in 0..channels {
            let mut buffer = VecDeque::with_capacity(max_delay_frames);
            for _ in 0..max_delay_frames {
                buffer.push_front(0.0);
            }
            buffers.push(buffer);
        }
        Delay {
            buffers,
            frame_number: 0,
            mask,
            output: vec![0.0; channels as _],
            sample_rate,
        }
    }
}

impl Module for Delay {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (output, input, buffer) in izip!(
            self.output.iter_mut(),
            input.chunks(2),
            self.buffers.iter_mut()
        ) {
            let x = input[0];
            let z = input[1] * self.sample_rate;
            let delay = z as usize;
            let k = z.fract();
            let a = buffer[delay & self.mask];
            let b = buffer[(delay + 1) & self.mask];
            *output = (1.0 - k) * a + k * b;
            buffer.pop_back();
            buffer.push_front(x);
        }
        self.frame_number += 1;
    }
}
