use crate::csv::TrustGraph;
use std::collections::{HashSet, HashMap};

pub trait MutualConnectionAnalysis {
    fn analyze_mutual_connections(&self) -> Vec<(u32, u32, u32)>;
    fn find_mutual_trust_score(&self) -> HashMap<(u32, u32), f64>;
}

impl MutualConnectionAnalysis for TrustGraph {
    fn analyze_mutual_connections(&self) -> Vec<(u32, u32, u32)> {
        let mut mutual_results = Vec::new();
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

    fn find_mutual_trust_score(&self) -> HashMap<(u32, u32), f64> {
        let mut mutual_trust_scores = HashMap::new();
    
        for node_index in self.graph.node_indices() {
            // Find all neighbors of this node
            let neighbors: HashSet<_> = self.graph.neighbors(node_index).collect();
    
            // For each pair of neighbors, calculate their mutual trust score
            for &neighbor1 in &neighbors {
                for &neighbor2 in &neighbors {
                    if neighbor1 != neighbor2 {
                        // Average trust score of neighbor pairs
                        let trust1 = self.graph.find_edge(node_index, neighbor1)
                            .and_then(|edge| Some(*self.graph.edge_weight(edge)? as f64));
                        let trust2 = self.graph.find_edge(node_index, neighbor2)
                            .and_then(|edge| Some(*self.graph.edge_weight(edge)? as f64));
    
                        if let (Some(trust1), Some(trust2)) = (trust1, trust2) {
                            let average_trust = (trust1 + trust2) / 2.0;
                            mutual_trust_scores.insert((self.graph[neighbor1], self.graph[neighbor2]), average_trust);
                        }
                    }
                }
            }
        }
    
        mutual_trust_scores
    }

}
