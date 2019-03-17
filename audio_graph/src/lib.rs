//! # audio_graph
//!
//! audio_graph is a library which allows creating and sampling a network of interconnected
//! audio signal `Module`s.

mod graph;
mod module;
mod sample;
pub mod stack;

pub use crate::graph::{AudioGraph, AudioNode};
pub use crate::module::Module;
pub use sample::{Frame, Sample};
