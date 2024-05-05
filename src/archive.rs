mod graph;
use crate::graph::TrustGraph;
use std::error::Error;

use serde::Deserialize;
use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::io;
use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{Bfs, Visitable};
use petgraph::visit::EdgeRef;
use petgraph::EdgeDirection;

#[derive(Debug, Deserialize)]
pub struct TrustRelation {
    from: u32,
    to: u32,
    weight: f64,
}

#[derive(Debug)]
pub struct TrustGraph {
    graph: DiGraph<u32, i32>,  // Node is user ID, edge weight is the trust value
    node_indices: HashMap<u32, NodeIndex>,
}

impl TrustGraph {
    pub fn new() -> Self {
        TrustGraph {
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
        }
    }

    // pub fn add_edge(&mut self, a: NodeIndex<Ix>, b: NodeIndex<Ix>, weight: E) -> EdgeIndex<Ix> { }

    pub fn add_relation(&mut self, relation: &TrustRelation) {
        let TrustRelation { from, to, weight } = *relation;
        let from_index = *self.node_indices.entry(from).or_insert_with(|| self.graph.add_node(from));
        let to_index = *self.node_indices.entry(to).or_insert_with(|| self.graph.add_node(to));

        self.graph.add_edge(from_index, to_index, weight);
    }

    pub fn from_csv(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut graph = TrustGraph::new();

        for result in rdr.deserialize() {
            let relation: TrustRelation = result?;
            graph.add_relation(&relation);
        }

        Ok(graph)
    }

    pub fn bfs_trusted_transactions(&self, start_id: u32) {
        if let Some(&start_index) = self.node_indices.get(&start_id) {
            let mut bfs = Bfs::new(&self.graph, start_index);
            while let Some(nx) = bfs.next(&self.graph) {
                for edge in self.graph.edges(nx) {
                    if *edge.weight() > 0 {
                        println!("Trusted transaction from {} to {}", self.graph[edge.source()], self.graph[edge.target()]);
                    }
                }
            }
        }
    }
}



fn main() -> Result<(), Box<dyn Error>> {
    let trust_graph = TrustGraph::from_csv(r"C:\Users\nicho\projects\ds210proj\BTCAlphaNet.csv")?;
    trust_graph.bfs_trusted_transactions(1);  // Start BFS from node with ID 1
    Ok(())
}