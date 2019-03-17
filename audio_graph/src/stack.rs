use crate::graph::{AudioGraph, AudioNode};

pub enum Op {
    Connect(AudioNode),
    Pop,
    Swap,
    Dup,
    Rot,
}

#[derive(Debug)]
pub enum Error {
    StackExhausted(usize),
}

pub fn build_graph(mut ops: Vec<Op>, channels: u8, inputs: u8) -> Result<AudioGraph, Error> {
    let mut graph = AudioGraph::new(channels, inputs);
    let mut stack = Vec::new();
    for (i, op) in ops.drain(..).enumerate() {
        match op {
            Op::Connect(node) => {
                let inputs = node.inputs() as usize;
                if stack.len() < inputs {
                    return Err(Error::StackExhausted(i));
                }
                let idx = graph.add_node(node);
                let start = stack.len() - inputs;
                graph.set_sources(idx, &stack.drain(start..).collect::<Vec<_>>());
                stack.push(idx);
            }
            Op::Pop => {
                let _ = stack.pop();
            }
            Op::Dup => match stack.last() {
                Some(idx) => stack.push(*idx),
                None => return Err(Error::StackExhausted(i)),
            },
            Op::Swap => {
                let len = stack.len();
                if len < 2 {
                    return Err(Error::StackExhausted(i));
                }
                stack.swap(len - 2, len - 1);
            }
            Op::Rot => {
                let len = stack.len();
                if len < 3 {
                    return Err(Error::StackExhausted(i));
                }
                stack.swap(len - 2, len - 1);
                stack.swap(len - 3, len - 1);
            }
        }
    }
    Ok(graph)
}
