use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::Deserialize;
use nalgebra::DMatrix;

#[derive(Debug, Clone, Deserialize)]
pub struct TrustRelation {
    pub rater: u32,
    pub ratee: u32,
    pub trust_score: i32,
}

#[derive(Debug, Clone)]
pub struct TrustGraph {
    pub graph: DiGraph<u32, i32>,
    pub node_indices: HashMap<u32, NodeIndex>,
}

impl TrustGraph {
    pub fn new() -> Self {
        TrustGraph {
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
        }
    }

    pub fn add_relation(&mut self, relation: TrustRelation) {
        let from_index = *self.node_indices.entry(relation.rater)
            .or_insert_with(|| self.graph.add_node(relation.rater));
        let to_index = *self.node_indices.entry(relation.ratee)
            .or_insert_with(|| self.graph.add_node(relation.ratee));
        self.graph.add_edge(from_index, to_index, relation.trust_score);
    }

    pub fn add_relations(&mut self, relations: Vec<TrustRelation>) {
        for relation in relations {
            self.add_relation(relation);
        }
    }
    pub fn build_adjacency_matrix(&self) -> DMatrix<f64> {
        let num_nodes = self.graph.node_count();
        let mut matrix = DMatrix::zeros(num_nodes, num_nodes);
        for edge in self.graph.edge_references() {
            let source_index = edge.source().index();
            let target_index = edge.target().index();
            matrix[(source_index, target_index)] = *edge.weight() as f64;
        }
        matrix
    }
}

pub fn read_csv(filepath: &str) -> Result<Vec<TrustRelation>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut relations = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if index == 0 { // to skip the header row
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let rater = parts[0].parse().unwrap_or(0);
            let ratee = parts[1].parse().unwrap_or(0);
            let trust_score = parts[2].parse().unwrap_or(0);

            relations.push(TrustRelation {
                rater,
                ratee,
                trust_score,
            });
        }
    }
    Ok(relations)
}
