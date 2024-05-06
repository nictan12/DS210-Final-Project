use std::collections::VecDeque;
use csv_reader_and_graph::TrustGraph;

pub trait MutualConnectionAnalysis {
    fn analyze_mutual_connections(&self) -> Vec<(u32, u32, u32)>;
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
}
