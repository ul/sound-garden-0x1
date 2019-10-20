//! # Audio graph
use crate::module::Module;
use crate::sample::{Frame, Sample};
use fixedbitset::FixedBitSet;
use petgraph::algo::{toposort, DfsSpace};
use petgraph::prelude::*;

pub type AudioNode = Box<dyn Module + Send>;

/// Structure which manages network of Modules.
pub struct AudioGraph {
    /// Nodes are boxed Modules and edges represent source->sink connections.
    graph: StableGraph<AudioNode, ()>,
    /// `sample` writes output from source nodes into this buffer and then passes it to sink.
    /// Buffer is reused during graph traversal and between samples to avoid memory allocations.
    input: Vec<Sample>,
    /// How many external inputs does graph expect.
    inputs: u8,
    /// `sample` walks graph in topological order which is cached here.
    order: Vec<NodeIndex>,
    /// Output of the graph's last module.
    output: Vec<Sample>,
    /// Workspace for topological sort is stored in structure for re-use.
    space: DfsSpace<NodeIndex, FixedBitSet>,
}

/// Maximum number of sources to connect to sink.
/// This number is required because input buffer is allocated during AudioGraph initialization and
/// then re-used across all nodes sampling. There is no real need to have it hardcoded though.
/// It can be made an argument of AudioGraph::new.
const MAX_SOURCES: usize = 16;

impl AudioGraph {
    pub fn new(channels: u8, inputs: u8) -> Self {
        let graph = StableGraph::default();
        let space = DfsSpace::new(&graph);
        AudioGraph {
            graph,
            input: vec![0.0; MAX_SOURCES * channels as usize],
            inputs,
            order: Vec::new(),
            output: vec![0.0; channels as _],
            space,
        }
    }

    pub fn node(&self, idx: NodeIndex) -> &AudioNode {
        &self.graph[idx]
    }

    /// Add node to the graph and return index assigned to the node.
    /// This index is stable and could be used to reference the node when building connections.
    pub fn add_node(&mut self, n: AudioNode) -> NodeIndex {
        self.graph.add_node(n)
    }

    /// Connect nodes in a chain, from left to right.
    /// It clears nodes' sources before connecting, except for the first one.
    pub fn chain(&mut self, nodes: &[NodeIndex]) {
        for pair in nodes.windows(2) {
            let source = pair[0];
            let sink = pair[1];
            self.clear_sources(sink);
            self.graph.update_edge(source, sink, ());
        }
        self.update_order();
    }

    /// Set node `a` as a single source of node `b`.
    /// It clears `b`'s sources before connecting, to set multiple sources use `set_sources`.
    pub fn connect(&mut self, a: NodeIndex, b: NodeIndex) {
        self.clear_sources(b);
        self.graph.update_edge(a, b, ());
        self.update_order();
    }

    /// Set multiple sources for the `sink` node.
    /// It clears `sink`'s sources before connecting.
    /// `source`s' outputs are layouted in `sink` input buffer in the provided order.
    /// Ref `Module::sample` doc for an example of input layout.
    pub fn set_sources(&mut self, sink: NodeIndex, sources: &[NodeIndex]) {
        self.clear_sources(sink);
        // ref `sample` method comments for the reason of reversing sources
        for source in sources.iter().rev() {
            self.graph.update_edge(*source, sink, ());
        }
        self.update_order();
    }

    pub fn set_sources_rev(&mut self, sink: NodeIndex, sources: &[NodeIndex]) {
        self.clear_sources(sink);
        // ref `sample` method comments for the reason of reversing sources
        for source in sources.iter() {
            self.graph.update_edge(*source, sink, ());
        }
        self.update_order();
    }

    pub fn clear(&mut self) {
        self.order.clear();
        self.graph.clear();
    }

    /// Update node traversal order.
    /// It must be called after any connection change.
    pub fn update_order(&mut self) {
        self.order = toposort(&self.graph, Some(&mut self.space)).unwrap_or_else(|_| vec![]);
    }

    /// Remove all incoming connections of the node.
    fn clear_sources(&mut self, sink: NodeIndex) {
        while let Some(edge) = self
            .graph
            .neighbors_directed(sink, Incoming)
            .detach()
            .next_edge(&self.graph)
        {
            self.graph.remove_edge(edge);
        }
    }
}

impl Module for AudioGraph {
    fn inputs(&self) -> u8 {
        self.inputs
    }

    fn output(&self) -> &Frame {
        &self.output
    }

    fn sample(&mut self, input: &Frame) {
        for idx in self.order.iter().map(|x| *x) {
            let graph = &mut self.graph;
            let node_inputs = graph[idx].inputs() as usize;
            if node_inputs > 0 {
                // NOTE neighbors_directed walks edges starting from the most recently added (is it
                // guaranteed?). This is the reason why connection methods (connect, set_sources,
                // chain etc.) call clear_sources first and reverse sources. Always resetting
                // sources instead of finer-grained manipulation reduces risk of confusing their
                // order. We might want to consider to name edges and pass HashMap instead instead
                // of Vec as input. But it implies non-neglegible performance hit.
                //
                // Ref `Module::sample` doc for an example of input layout.
                for (i, source) in graph.neighbors_directed(idx, Incoming).enumerate() {
                    for (channel, sample) in graph[source].output().iter().enumerate() {
                        self.input[i + channel * node_inputs] = *sample;
                    }
                }
                graph[idx].sample(&self.input);
            } else {
                // If node does not have any inputs it might be waiting for external input.
                graph[idx].sample(input);
            }
        }
        if let Some(&node) = self.order.last() {
            self.output.copy_from_slice(self.graph[node].output());
        }
    }
}
