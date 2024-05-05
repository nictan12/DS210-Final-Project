use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::Deserialize;
use std::collections::HashMap;
use nalgebra::{DMatrix, DVector};

#[derive(Debug, Deserialize)]
struct TrustRelation {
    rater: u32,
    ratee: u32,
    trust_score: i32,
}

struct TrustGraph {
    graph: DiGraph<u32, i32>,
    node_indices: HashMap<u32, NodeIndex>,
}

impl TrustGraph {
    fn new() -> Self {
        TrustGraph {
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
        }
    }

    fn add_relation(&mut self, relation: TrustRelation) {
        let from_index = *self.node_indices.entry(relation.rater).or_insert_with(|| self.graph.add_node(relation.rater));
        let to_index = *self.node_indices.entry(relation.ratee).or_insert_with(|| self.graph.add_node(relation.ratee));
        self.graph.add_edge(from_index, to_index, relation.trust_score);
    }

    fn build_adjacency_matrix(&self) -> DMatrix<f64> {
        let num_nodes = self.graph.node_count();
        let mut matrix = DMatrix::zeros(num_nodes, num_nodes);
        for edge in self.graph.edge_references() {
            let source_index = edge.source().index();
            let target_index = edge.target().index();
            matrix[(source_index, target_index)] = *edge.weight() as f64;
        }
        matrix
    }

    fn compute_eigenvector_centrality(&self, max_iter: usize, tolerance: f64) -> HashMap<u32, f64> {
        let adjacency_matrix = self.build_adjacency_matrix();
        let num_nodes = adjacency_matrix.nrows();
        let mut eigenvector = DVector::from_element(num_nodes, 1.0 / num_nodes as f64);

        for _ in 0..max_iter {
            let next_vector = &adjacency_matrix * &eigenvector;
            let norm = next_vector.norm();
            let normalized_vector = next_vector / norm;

            if (normalized_vector - eigenvector).norm() < tolerance {
                break;
            }
            eigenvector = normalized_vector;
        }

        let mut centrality = HashMap::new();
        for (node, &score) in self.graph.node_indices().iter() {
            centrality.insert(*node, eigenvector[self.graph[*score].index()]);
        }
        centrality
    }
}
fn analyze_mutual_connections(&self) -> Vec<(u32, u32, u32)> {
    let mut mutual_results = Vec::new();

    // Iterate through all nodes to find mutual neighbors
    for node_index in self.graph.node_indices() {
        let neighbors: HashSet<_> = self.graph.neighbors(node_index).collect();
        for &neighbor1 in &neighbors {
            for &neighbor2 in &neighbors {
                if neighbor1 != neighbor2 {
                    let edge1_to_2 = self.graph.find_edge(neighbor1, neighbor2);
                    let edge2_to_1 = self.graph.find_edge(neighbor2, neighbor1);

                    if edge1_to_2.is_none() && edge2_to_1.is_none() {
                        let node_id = self.graph[node_index];
                        let neighbor1_id = self.graph[neighbor1];
                        let neighbor2_id = self.graph[neighbor2];

                        mutual_results.push((node_id, neighbor1_id, neighbor2_id));
                        println!(
                            "Mutual connection found through node {} between {} and {}",
                            node_id, neighbor1_id, neighbor2_id
                        );
                    }
                }
            }
        }
    }

    mutual_results
}
fn load_csv_to_graph(filepath: &str) -> TrustGraph {
    let mut rdr = csv::Reader::from_path(filepath).expect("Cannot read CSV file");
    let mut graph = TrustGraph::new();

    for result in rdr.deserialize() {
        let relation: TrustRelation = result.expect("Invalid CSV row");
        graph.add_relation(relation);
    }

    graph
}

fn main() {
    let trust_graph = load_csv_to_graph(r"C:\Users\nicho\projects\ds210final\DS210-Final-Project\BTCAlphaNet.csv");
    let centrality = trust_graph.compute_eigenvector_centrality(100, 1e-6);
    for (user, score) in centrality {
        println!("User {}: Centrality Score = {}", user, score);
    }
}
