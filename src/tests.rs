use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};

// Trust relation structure
#[derive(Debug)]
struct TrustRelation {
    rater: u32,
    ratee: u32,
    trust_score: i32,
}

// Trust graph structure
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
        let from_index = *self.node_indices.entry(relation.rater)
            .or_insert_with(|| self.graph.add_node(relation.rater));
        let to_index = *self.node_indices.entry(relation.ratee)
            .or_insert_with(|| self.graph.add_node(relation.ratee));
        self.graph.add_edge(from_index, to_index, relation.trust_score);
    }

    // Dummy implementation for demonstration
    fn compute_eigenvector_centrality(&self) -> HashMap<u32, f64> {
        let mut centrality = HashMap::new();
        for &node in self.node_indices.keys() {
            centrality.insert(node, 1.0); // Example: give a fixed score of 1.0
        }
        centrality
    }

    fn compute_trust_score_between(&self, a: u32, b: u32) -> Option<f64> {
        // Dummy implementation just to illustrate
        if a != b {
            Some(5.0) // Return a fixed score
        } else {
            None
        }
    }
}

// Testing the TrustGraph functionality
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_structure() {
        // Initialize the graph and add relations
        let mut graph = TrustGraph::new();
        graph.add_relation(TrustRelation { rater: 1, ratee: 2, trust_score: 10 });
        graph.add_relation(TrustRelation { rater: 2, ratee: 3, trust_score: 20 });
        graph.add_relation(TrustRelation { rater: 1, ratee: 3, trust_score: 15 });

        // Ensure that all nodes are present
        assert_eq!(graph.node_indices.len(), 3);

        // Check specific edges
        assert!(graph.graph.find_edge(*graph.node_indices.get(&1).unwrap(), *graph.node_indices.get(&2).unwrap()).is_some());
        assert!(graph.graph.find_edge(*graph.node_indices.get(&2).unwrap(), *graph.node_indices.get(&3).unwrap()).is_some());
    }

    #[test]
    fn test_eigenvector_centrality() {
        // Initialize the graph and add relations
        let mut graph = TrustGraph::new();
        graph.add_relation(TrustRelation { rater: 1, ratee: 2, trust_score: 10 });
        graph.add_relation(TrustRelation { rater: 2, ratee: 3, trust_score: 20 });
        graph.add_relation(TrustRelation { rater: 1, ratee: 3, trust_score: 15 });

        // Compute eigenvector centrality
        let centrality = graph.compute_eigenvector_centrality();

        // Ensure that all expected nodes are present with correct scores
        assert!(centrality.get(&1).is_some());
        assert!(centrality.get(&2).is_some());
        assert!(centrality.get(&3).is_some());
    }

    #[test]
    fn test_mutual_scores() {
        // Initialize the graph and add relations
        let mut graph = TrustGraph::new();
        graph.add_relation(TrustRelation { rater: 1, ratee: 2, trust_score: 10 });
        graph.add_relation(TrustRelation { rater: 2, ratee: 3, trust_score: 20 });
        graph.add_relation(TrustRelation { rater: 1, ratee: 3, trust_score: 15 });

        // Test computation of mutual scores
        let trust_score = graph.compute_trust_score_between(1, 2);
        assert_eq!(trust_score, Some(5.0));

        let trust_score = graph.compute_trust_score_between(1, 3);
        assert_eq!(trust_score, Some(5.0));
    }
}
