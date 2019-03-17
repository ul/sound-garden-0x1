//! # Convolution
//!
//! Convolve two signals by making dot-product of a 3-sample sliding window on both.
//!
//! Sources to connect: input and kernel, but roles are vague in this case.
use audio_graph::{Frame, Module, Sample};
use std::collections::VecDeque;

pub struct Convolution {
    windows: Vec<VecDeque<Sample>>,
    output: Vec<Sample>,
}

impl Convolution {
    pub fn new(channels: u8, window_size: usize) -> Self {
        let channels = channels as usize;
        let mut windows = Vec::with_capacity(channels);
        for _ in 0..channels {
            let mut window = VecDeque::with_capacity(window_size);
            for _ in 0..window_size {
                window.push_back(0.0);
            }
            windows.push(window);
        }
        Convolution {
            output: vec![0.0; channels],
            windows,
        }
    }
}

impl Module for Convolution {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (output, input, window) in izip!(
            self.output.iter_mut(),
            input.chunks(2),
            self.windows.iter_mut()
        ) {
            window.pop_front();
            window.push_back(input.iter().product());
            *output = window.iter().sum();
        }
    }
}

pub struct ConvolutionM {
    window_size: usize,
    windows: Vec<VecDeque<Sample>>,
    output: Vec<Sample>,
}

impl ConvolutionM {
    pub fn new(channels: u8, window_size: usize) -> Self {
        let channels = channels as usize;
        let mut windows = Vec::with_capacity(channels);
        for _ in 0..channels {
            let mut window = VecDeque::with_capacity(3);
            for _ in 0..window_size {
                window.push_back(0.0);
            }
            windows.push(window);
        }
        ConvolutionM {
            output: vec![0.0; channels],
            window_size,
            windows,
        }
    }
}

impl Module for ConvolutionM {
    fn inputs(&self) -> u8 {
        1 + self.window_size as u8
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (output, input, window) in izip!(
            self.output.iter_mut(),
            input.chunks(1 + self.window_size),
            self.windows.iter_mut()
        ) {
            window.pop_front();
            window.push_back(input[0]);
            let mut result = 0.0;
            for (sample, seed) in window.iter().zip(&input[1..]) {
                result += sample * seed;
            }
            *output = result;
        }
    }
}
