use audio_graph::{Frame, Module, Sample};

pub struct Metro {
    output: Vec<Sample>,
    last_trigger: Vec<u64>,
    frame_number: u64,
    sample_rate: Sample,
}

impl Metro {
    pub fn new(channels: u8, sample_rate: u32) -> Self {
        Metro {
            output: vec![0.0; channels as _],
            last_trigger: vec![0; channels as _],
            frame_number: 0,
            sample_rate: Sample::from(sample_rate),
        }
    }
}

impl Module for Metro {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (output, frequency, last_trigger) in
            izip!(self.output.iter_mut(), input, self.last_trigger.iter_mut())
        {
            let delta = self.sample_rate / frequency;
            *output = if delta as u64 <= self.frame_number - *last_trigger {
                *last_trigger = self.frame_number;
                1.0
            } else {
                0.0
            };
        }
        self.frame_number += 1;
    }
}

pub struct DMetro {
    output: Vec<Sample>,
    last_trigger: Vec<u64>,
    frame_number: u64,
    sample_rate: Sample,
}

impl DMetro {
    pub fn new(channels: u8, sample_rate: u32) -> Self {
        DMetro {
            output: vec![0.0; channels as _],
            last_trigger: vec![0; channels as _],
            frame_number: 0,
            sample_rate: Sample::from(sample_rate),
        }
    }
}

impl Module for DMetro {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (output, dt, last_trigger) in
            izip!(self.output.iter_mut(), input, self.last_trigger.iter_mut(),)
        {
            let delta = self.sample_rate * dt;
            *output = if delta as u64 <= self.frame_number - *last_trigger {
                *last_trigger = self.frame_number;
                1.0
            } else {
                0.0
            };
        }
        self.frame_number += 1;
    }
}

pub struct MetroHold {
    output: Vec<Sample>,
    frequencies: Vec<Sample>,
    last_trigger: Vec<u64>,
    frame_number: u64,
    sample_rate: Sample,
}

impl MetroHold {
    pub fn new(channels: u8, sample_rate: u32) -> Self {
        MetroHold {
            output: vec![0.0; channels as _],
            frequencies: vec![0.0; channels as _],
            last_trigger: vec![0; channels as _],
            frame_number: 0,
            sample_rate: Sample::from(sample_rate),
        }
    }
}

impl Module for MetroHold {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (output, frequency, last_trigger, last_frequency) in izip!(
            self.output.iter_mut(),
            input,
            self.last_trigger.iter_mut(),
            self.frequencies.iter_mut()
        ) {
            if *last_frequency == 0.0 {
                *last_frequency = *frequency
            }
            let delta = self.sample_rate / *last_frequency;
            *output = if delta as u64 <= self.frame_number - *last_trigger {
                *last_trigger = self.frame_number;
                *last_frequency = *frequency;
                1.0
            } else {
                0.0
            };
        }
        self.frame_number += 1;
    }
}

pub struct DMetroHold {
    output: Vec<Sample>,
    dts: Vec<Sample>,
    last_trigger: Vec<u64>,
    frame_number: u64,
    sample_rate: Sample,
}

impl DMetroHold {
    pub fn new(channels: u8, sample_rate: u32) -> Self {
        DMetroHold {
            output: vec![0.0; channels as _],
            dts: vec![0.0; channels as _],
            last_trigger: vec![0; channels as _],
            frame_number: 0,
            sample_rate: Sample::from(sample_rate),
        }
    }
}

impl Module for DMetroHold {
    fn inputs(&self) -> u8 {
        1
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (output, dt, last_trigger, last_dt) in izip!(
            self.output.iter_mut(),
            input,
            self.last_trigger.iter_mut(),
            self.dts.iter_mut()
        ) {
            if *last_dt == 0.0 {
                *last_dt = *dt
            }
            let delta = self.sample_rate * *last_dt;
            *output = if delta as u64 <= self.frame_number - *last_trigger {
                *last_trigger = self.frame_number;
                *last_dt = *dt;
                1.0
            } else {
                0.0
            };
        }
        self.frame_number += 1;
    }
}
