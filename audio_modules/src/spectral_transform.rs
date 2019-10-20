//! # Spectral transform
//!
//! Do a FFT of the input signal, transform bins, and produce an output signal with IFFT.
//!
//! Source to connect: input.
//!
//! TODO: make a generic do-something-with-spectrum module for easier experiments.
use audio_graph::{Frame, Module, Sample};
use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFT;
use std::collections::VecDeque;

pub struct SpectralTransform {
    input_buffers: Vec<VecDeque<Complex<Sample>>>,
    input_scratch: Vec<Complex<Sample>>,
    freq_buffer: Vec<Complex<Sample>>,
    output_buffers: Vec<Vec<Complex<Sample>>>,
    fft: Radix4<Sample>,
    ifft: Radix4<Sample>,
    output: Vec<Sample>,
    norm: Sample,
    period_mask: usize,
    period_offset: usize,
    window: Vec<Complex<Sample>>,
    frame_number: usize,
    transform: Box<dyn FnMut(&mut Vec<Complex<Sample>>) + Send>,
}

impl SpectralTransform {
    // window_size = 2048
    // period = 64
    pub fn new(
        channels: u8,
        window_size: usize,
        period: usize,
        transform: Box<dyn FnMut(&mut Vec<Complex<Sample>>) + Send>,
    ) -> Self {
        let channels = channels as usize;
        let mut input_buffers = Vec::with_capacity(channels);
        let mut output_buffers = Vec::with_capacity(channels);
        for _ in 0..channels {
            let mut window = VecDeque::with_capacity(window_size);
            for _ in 0..window_size {
                window.push_back(Complex::zero());
            }
            input_buffers.push(window);
            output_buffers.push(vec![Complex::zero(); window_size]);
        }
        let fft = Radix4::new(window_size, false);
        let ifft = Radix4::new(window_size, true);
        SpectralTransform {
            input_buffers,
            input_scratch: vec![Complex::zero(); window_size],
            freq_buffer: vec![Complex::zero(); window_size],
            output_buffers,
            fft,
            ifft,
            output: vec![0.0; channels],
            norm: (window_size as Sample).recip(),
            period_mask: period - 1,
            period_offset: window_size - period,
            frame_number: 0,
            window: apodize::hanning_iter(window_size)
                .map(Complex::from)
                .collect(),
            transform,
        }
    }
}

impl Module for SpectralTransform {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        let index = self.frame_number & self.period_mask;
        for (output, input, input_buffer, output_buffer) in izip!(
            self.output.iter_mut(),
            input,
            self.input_buffers.iter_mut(),
            self.output_buffers.iter_mut()
        ) {
            if index == 0 {
                let mut scratch = &mut self.input_scratch;
                let freq_buffer = &mut self.freq_buffer;
                let input_slices = input_buffer.as_slices();
                let n = input_slices.0.len();
                scratch[..n].clone_from_slice(input_slices.0);
                scratch[n..].clone_from_slice(input_slices.1);
                for (x, a) in scratch.iter_mut().zip(&self.window) {
                    *x *= a;
                }
                self.fft.process(&mut scratch, freq_buffer);
                (self.transform)(freq_buffer);
                self.ifft.process(freq_buffer, output_buffer);
            }
            *output = self.norm * output_buffer[self.period_offset + index].re;
            input_buffer.pop_front();
            input_buffer.push_back(Complex::from(input));
        }
        self.frame_number += 1;
    }
}
