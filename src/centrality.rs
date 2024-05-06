use crate::csv::TrustGraph;
use std::collections::HashMap;
use nalgebra::DVector;
use petgraph::adj::IndexType;

pub trait EigenvectorCentrality {
    fn compute_eigenvector_centrality(&self, max_iter: usize, tolerance: f64) -> HashMap<u32, f64>;
}

impl EigenvectorCentrality for TrustGraph {
    fn compute_eigenvector_centrality(&self, max_iter: usize, tolerance: f64) -> HashMap<u32, f64> {
        let adjacency_matrix = self.build_adjacency_matrix();
        let num_nodes = adjacency_matrix.nrows();
        let mut eigenvector = DVector::from_element(num_nodes, 1.0 / num_nodes as f64);

        for _ in 0..max_iter {
            let next_vector = &adjacency_matrix * &eigenvector;
            let norm = next_vector.norm();
            let normalized_vector = next_vector / norm;
            // Check convergence
            if (normalized_vector.clone() - eigenvector.clone()).norm() < tolerance {
                break;
            }

            eigenvector = normalized_vector;
        }

        let mut centrality = HashMap::new();
        for node in self.node_indices.keys() {
            if let Some(index) = self.node_indices.get(node) {
                centrality.insert(*node, eigenvector[self.graph[*index].index()]);
            }
        }
        centrality
    }
}
