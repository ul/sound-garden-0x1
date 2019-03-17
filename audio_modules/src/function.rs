//! # Functions
//!
//! Fn*N* modules allow to use regular numeric functions to transform input of *N* sources.
//!
//! Sources to connect: *N*, one for each argument of pure function.
use audio_graph::{Frame, Module, Sample};

pub struct Fn1 {
    ys: Vec<Sample>,
    f: fn(Sample) -> Sample,
}

impl Fn1 {
    pub fn new(channels: u8, f: fn(Sample) -> Sample) -> Self {
        Fn1 {
            ys: vec![0.0; channels as _],
            f,
        }
    }
}

impl Module for Fn1 {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.ys
    }

    fn sample(&mut self, input: &Frame) {
        for (y, x) in self.ys.iter_mut().zip(input) {
            *y = (self.f)(*x);
        }
    }
}

pub struct Fn2 {
    ys: Vec<Sample>,
    f: fn(Sample, Sample) -> Sample,
}

impl Fn2 {
    pub fn new(channels: u8, f: fn(Sample, Sample) -> Sample) -> Self {
        Fn2 {
            ys: vec![0.0; channels as _],
            f,
        }
    }
}

impl Module for Fn2 {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.ys
    }

    fn sample(&mut self, input: &Frame) {
        for (y, x) in self.ys.iter_mut().zip(input.chunks(2)) {
            *y = (self.f)(x[0], x[1]);
        }
    }
}

pub struct Fn3 {
    ys: Vec<Sample>,
    f: fn(Sample, Sample, Sample) -> Sample,
}

impl Fn3 {
    pub fn new(channels: u8, f: fn(Sample, Sample, Sample) -> Sample) -> Self {
        Fn3 {
            ys: vec![0.0; channels as _],
            f,
        }
    }
}

impl Module for Fn3 {
    fn inputs(&self) -> u8 {
        3
    }

    fn output(&self) -> &Frame {
        &self.ys
    }

    fn sample(&mut self, input: &Frame) {
        for (y, x) in self.ys.iter_mut().zip(input.chunks(3)) {
            *y = (self.f)(x[0], x[1], x[3]);
        }
    }
}
