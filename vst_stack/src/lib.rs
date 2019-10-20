#[macro_use]
extern crate objc;

mod context;
mod editor;

use audio_graph::{AudioGraph, Module, Sample};
use audio_stack::parse_graph;
use chrono::prelude::*;
use context::Context;
use editor::Editor;
use notify::{watcher, DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use parking_lot::Mutex;
use rand::random;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::Duration;
use vst::plugin::{Info, Plugin};

const CHANNELS: u8 = 2;
const PARAMETERS: u8 = 16;

struct SoundGarden {
    context: Arc<Mutex<Context>>,
    editor: Editor,
    graph: Arc<Mutex<AudioGraph>>,
    input: Vec<Sample>,
    parameters: Vec<f64>,
    _watcher: RecommendedWatcher,
}

impl Default for SoundGarden {
    fn default() -> Self {
        let dt = Local::now();
        let hash = format!("{:x}", random::<u64>());
        let mut source_dir = dirs::home_dir().unwrap();
        source_dir.push("SoundGarden");
        let source_dir = source_dir.to_str().unwrap().to_string();
        let source_path = format!(
            "{}/{}-{}.sg",
            source_dir,
            dt.format("%Y-%m-%d").to_string(),
            hash
        );
        println!("{}", source_path);
        let context = Arc::new(Mutex::new(Context {
            channels: CHANNELS,
            sample_rate: 48_000,
            parameters: PARAMETERS,
        }));
        let graph = Arc::new(Mutex::new(AudioGraph::new(CHANNELS, CHANNELS + PARAMETERS)));
        let editor = Editor {
            path: source_path.clone(),
            is_open: false,
        };
        // Create a channel to receive the events.
        let (tx, rx) = channel();

        // Create a watcher object, delivering debounced events.
        // The notification back-end is selected based on the platform.
        let mut watcher = watcher(tx, Duration::from_millis(1)).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher
            .watch(source_dir, RecursiveMode::NonRecursive)
            .unwrap();

        let new_graph = graph.clone();
        let ctx = context.clone();
        std::thread::spawn(move || {
            let mut source_code = "".to_string();
            loop {
                match rx.recv() {
                    Ok(DebouncedEvent::Create(path)) | Ok(DebouncedEvent::Write(path)) => {
                        if !path.to_str().unwrap().contains(&hash) {
                            continue;
                        }
                        if let Ok(code) = std::fs::read_to_string(path) {
                            if source_code == code {
                                continue;
                            }
                            source_code = code;
                            let ctx = ctx.lock();
                            // TODO error reporting
                            if let Ok(graph) = parse_graph(
                                &source_code,
                                ctx.channels,
                                ctx.sample_rate,
                                ctx.channels + ctx.parameters,
                            ) {
                                *new_graph.lock() = graph;
                            }
                        }
                    }
                    Err(_) => break,
                    _ => continue,
                }
            }
            // let _ = std::fs::remove_file(source_path);
        });

        SoundGarden {
            context,
            editor,
            graph,
            input: vec![0.0; (CHANNELS + PARAMETERS) as _],
            parameters: vec![0.0; PARAMETERS as _],
            _watcher: watcher,
        }
    }
}

impl Plugin for SoundGarden {
    fn get_info(&self) -> Info {
        Info {
            name: "Sound Garden".to_string(),
            vendor: "Ruslan Prokopchuk".to_string(),
            unique_id: 1_804_198_801,
            inputs: i32::from(CHANNELS),
            outputs: i32::from(CHANNELS),
            f64_precision: true,
            parameters: i32::from(PARAMETERS), // param:<N>
            version: 1,
            category: vst::plugin::Category::Synth,
            ..Default::default()
        }
    }

    fn get_editor(&mut self) -> Option<&mut dyn vst::editor::Editor> {
        Some(&mut self.editor)
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.context.lock().sample_rate = rate as _;
    }

    fn can_be_automated(&self, _index: i32) -> bool {
        true
    }

    fn get_parameter(&self, index: i32) -> f32 {
        if index < i32::from(PARAMETERS) {
            self.parameters[index as usize] as f32
        } else {
            0.0
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        if index < i32::from(PARAMETERS) {
            self.parameters[index as usize] = Sample::from(value);
        }
    }

    // #[no_alloc]
    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        let (inputs, mut outputs) = buffer.split();

        // Iterate over inputs as (&f32, &f32)
        let (left, right) = inputs.split_at(1);
        let stereo_in = left[0].iter().zip(right[0].iter());

        // Iterate over outputs as (&mut f32, &mut f32)
        let (mut left, mut right) = outputs.split_at_mut(1);
        let stereo_out = left[0].iter_mut().zip(right[0].iter_mut());

        // Prepare parameters and graph
        self.input[CHANNELS as _..].clone_from_slice(&self.parameters);
        let mut g = self.graph.lock();

        // Zip and process
        for ((left_in, right_in), (left_out, right_out)) in stereo_in.zip(stereo_out) {
            self.input[0] = Sample::from(*left_in);
            self.input[1] = Sample::from(*right_in);
            g.sample(&self.input);
            let output = g.output();
            *left_out = output[0] as f32;
            *right_out = output[1] as f32;
        }
    }

    // #[no_alloc]
    fn process_f64(&mut self, buffer: &mut vst::buffer::AudioBuffer<f64>) {
        let (inputs, mut outputs) = buffer.split();

        // Iterate over inputs as (&f32, &f32)
        let (left, right) = inputs.split_at(1);
        let stereo_in = left[0].iter().zip(right[0].iter());

        // Iterate over outputs as (&mut f32, &mut f32)
        let (mut left, mut right) = outputs.split_at_mut(1);
        let stereo_out = left[0].iter_mut().zip(right[0].iter_mut());

        // Prepare parameters and graph
        self.input[CHANNELS as _..].clone_from_slice(&self.parameters);
        let mut g = self.graph.lock();

        // Zip and process
        for ((left_in, right_in), (left_out, right_out)) in stereo_in.zip(stereo_out) {
            self.input[0] = *left_in;
            self.input[1] = *right_in;
            g.sample(&self.input);
            let output = g.output();
            *left_out = output[0];
            *right_out = output[1];
        }
    }
}

vst::plugin_main!(SoundGarden);
