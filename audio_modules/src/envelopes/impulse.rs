use audio_graph::{Frame, Module, Sample};

pub struct Impulse {
    frame: usize,
    last_trigger: Vec<Sample>,
    output: Vec<Sample>,
    sample_period: Sample,
    trigger_frame: Vec<usize>,
}

impl Impulse {
    pub fn new(channels: u8, sample_rate: u32) -> Self {
        let channels = channels as usize;
        Impulse {
            frame: 0,
            last_trigger: vec![0.0; channels],
            output: vec![0.0; channels],
            sample_period: Sample::from(sample_rate).recip(),
            trigger_frame: vec![0; channels],
        }
    }
}

impl Module for Impulse {
    fn inputs(&self) -> u8 {
        2
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for (output, input, last_trigger, trigger_frame) in izip!(
            self.output.iter_mut(),
            input.chunks(2),
            self.last_trigger.iter_mut(),
            self.trigger_frame.iter_mut()
        ) {
            let trigger = input[0];
            let apex = input[1];
            if *last_trigger <= 0.0 && trigger > 0.0 {
                *trigger_frame = self.frame;
            }
            let time = (self.frame - *trigger_frame) as Sample * self.sample_period;
            let h = time / apex;
            *output = h * (1.0 - h).exp();
            *last_trigger = trigger;
        }
        self.frame += 1;
    }
}
