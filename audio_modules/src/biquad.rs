//! BiQuad Filters
//!
//! Sources to connect: input, cut-off frequency, Q.
use audio_graph::{Frame, Module, Sample};

type MakeCoefficients =
    fn(Sample, Sample, Sample) -> (Sample, Sample, Sample, Sample, Sample, Sample);

pub fn make_lpf_coefficients(
    _sin_o: Sample,
    cos_o: Sample,
    alpha: Sample,
) -> (Sample, Sample, Sample, Sample, Sample, Sample) {
    let b1 = 1.0 - cos_o;
    let b0 = 0.5 * b1;
    (b0, b1, b0, 1.0 + alpha, -2.0 * cos_o, 1.0 - alpha)
}

pub fn make_hpf_coefficients(
    _sin_o: Sample,
    cos_o: Sample,
    alpha: Sample,
) -> (Sample, Sample, Sample, Sample, Sample, Sample) {
    let k = 1.0 + cos_o;
    let b0 = 0.5 * k;
    let b1 = -k;
    (b0, b1, b0, 1.0 + alpha, -2.0 * cos_o, 1.0 - alpha)
}

pub struct BiQuad {
    make_coefficients: MakeCoefficients,
    output: Vec<Sample>,
    sample_angular_period: Sample,
    x1: Vec<Sample>,
    x2: Vec<Sample>,
    y2: Vec<Sample>,
}

impl BiQuad {
    pub fn new(channels: u8, sample_rate: u32, make_coefficients: MakeCoefficients) -> Self {
        let sample_angular_period = 2.0 * std::f64::consts::PI / Sample::from(sample_rate);
        BiQuad {
            make_coefficients,
            output: vec![0.0; channels as _],
            sample_angular_period,
            x1: vec![0.0; channels as _],
            x2: vec![0.0; channels as _],
            y2: vec![0.0; channels as _],
        }
    }
}

impl Module for BiQuad {
    fn inputs(&self) -> u8 {
        3
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (y, input, x1, x2, y2) in izip!(
            self.output.iter_mut(),
            input.chunks(3),
            self.x1.iter_mut(),
            self.x2.iter_mut(),
            self.y2.iter_mut()
        ) {
            let x = input[0];
            let frequency = input[1];
            let q = input[2];

            let y1 = *y;

            let o = frequency * self.sample_angular_period;
            let sin_o = o.sin();
            let cos_o = o.cos();
            let alpha = sin_o / (2.0 * q);
            let (b0, b1, b2, a0, a1, a2) = (self.make_coefficients)(sin_o, cos_o, alpha);
            *y = (x * b0 + *x1 * b1 + *x2 * b2 - y1 * a1 - *y2 * a2) / a0;

            *x2 = *x1;
            *x1 = x;
            *y2 = y1;
        }
    }
}
