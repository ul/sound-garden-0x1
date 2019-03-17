//! # Yin
//!
//! Yin pitch detection algorithm.
//!
//! Sources to connect: signal to detect pitch of.
//!
//! TODO use FFT and avoid O(n^2)
use audio_graph::{Frame, Module, Sample};
use std::collections::VecDeque;

pub struct Yin {
    buffer: Vec<Sample>,
    channels: usize,
    output: Vec<Sample>,
    period: usize,
    frame_number: usize,
    sample_rate: Sample,
    threshold: Sample,
    windows: Vec<VecDeque<Sample>>,
}

impl Yin {
    // window_size = 1024
    // threshold = 0.2
    pub fn new(
        channels: u8,
        sample_rate: u32,
        window_size: usize,
        period: usize,
        threshold: Sample,
    ) -> Self {
        let channels = channels as usize;
        let mut windows = Vec::with_capacity(channels);
        for _ in 0..channels {
            let mut window = VecDeque::with_capacity(window_size);
            for _ in 0..window_size {
                window.push_back(0.0);
            }
            windows.push(window);
        }
        Yin {
            buffer: vec![0.0; window_size / 2],
            channels,
            output: vec![0.0; channels],
            period,
            frame_number: 0,
            sample_rate: Sample::from(sample_rate),
            threshold,
            windows,
        }
    }

    fn difference(&mut self, channel: usize) {
        let window = &self.windows[channel];
        let buffer_len = self.buffer.len();
        for (tau, x) in self.buffer.iter_mut().enumerate().skip(1) {
            *x = 0.0;
            for i in 0..buffer_len {
                let delta = window[i] - window[i + tau];
                *x += delta * delta;
            }
        }
    }

    fn cumulative_mean_normalized_difference(&mut self) {
        let mut running_sum = 0.0;
        self.buffer[0] = 1.0;
        for (tau, x) in self.buffer.iter_mut().enumerate().skip(1) {
            running_sum += *x;
            *x *= tau as f64 / running_sum;
        }
    }

    fn absolute_threshold(&self) -> Option<usize> {
        // Search through the array of cumulative mean values, and look for ones that are below the threshold.
        // The first two positions in yinBuffer are always so start at the third (index 2).
        let mut tau = 2;
        let buffer_len = self.buffer.len();
        while tau < buffer_len && !(self.buffer[tau] < self.threshold) {
            tau += 1;
        }
        while tau + 1 < buffer_len && self.buffer[tau + 1] < self.buffer[tau] {
            tau += 1;
        }
        if tau == buffer_len || self.buffer[tau] >= self.threshold {
            None
        } else {
            Some(tau)
        }
    }

    fn parabolic_interpolation(&self, x1: usize) -> Sample {
        let x0 = x1 - 1;
        let x2 = x1 + 1;
        let s0 = self.buffer[x0];
        let s1 = self.buffer[x1];
        if x2 < self.buffer.len() {
            let s2 = self.buffer[x2];
            let d = 2.0 * s1 - s2 - s0;
            let delta = s2 - s0;
            x1 as Sample + if d != 0.0 { delta / (2.0 * d) } else { 0.0 }
        } else if s0 < s1 {
            x0 as Sample
        } else {
            x1 as Sample
        }
    }
}

impl Module for Yin {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (window, input) in self.windows.iter_mut().zip(input) {
            window.pop_front();
            window.push_back(*input);
        }
        if self.frame_number % self.period == 0 {
            for channel in 0..self.channels {
                self.difference(channel);
                self.cumulative_mean_normalized_difference();
                self.output[channel] = match self.absolute_threshold() {
                    Some(tau_estimate) => {
                        self.sample_rate / self.parabolic_interpolation(tau_estimate)
                    }
                    None => 0.0,
                }
            }
        }
        self.frame_number += 1;
    }
}
