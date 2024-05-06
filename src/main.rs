use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::{HashMap, HashSet};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::adj::IndexType;
use serde::Deserialize;
use nalgebra::{DMatrix, DVector};

#[derive(Debug, Deserialize)]
pub struct TrustRelation {
    rater: u32,
    ratee: u32,
    trust_score: i32,
}

#[derive(Debug, Clone)]
pub struct TrustGraph {
    graph: DiGraph<u32, i32>,
    node_indices: HashMap<u32, NodeIndex>,
}

impl TrustGraph {
    pub fn new() -> Self {
        TrustGraph {
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
        }
    }
    // Add a single trust relation to the graph
    pub fn add_relation(&mut self, relation: TrustRelation) {
        let from_index = *self.node_indices.entry(relation.rater).or_insert_with(|| self.graph.add_node(relation.rater));
        let to_index = *self.node_indices.entry(relation.ratee).or_insert_with(|| self.graph.add_node(relation.ratee));
        self.graph.add_edge(from_index, to_index, relation.trust_score);
    }
     // Add a list of relations to the graph
     fn add_relations(&mut self, relations: Vec<TrustRelation>) {
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

    pub fn compute_eigenvector_centrality(&self, max_iter: usize, tolerance: f64) -> HashMap<u32, f64> {
        let adjacency_matrix = self.build_adjacency_matrix();
        let num_nodes = adjacency_matrix.nrows();
        let mut eigenvector = DVector::from_element(num_nodes, 1.0 / num_nodes as f64);

        for _ in 0..max_iter {
            let next_vector = &adjacency_matrix * &eigenvector;
            let norm = next_vector.norm();
            let normalized_vector = next_vector / norm;
            // checking for convergence 
            if (normalized_vector.clone() - eigenvector.clone()).norm() < tolerance {
                break;
            }
            eigenvector = normalized_vector;
        }

        let mut centrality = HashMap::new();
        for node in self.node_indices.keys() {
            if let Some(index) = self.node_indices.get(node) {
                // Insert the node's centrality score into the HashMap
                centrality.insert(*node, eigenvector[self.graph[*index].index()]);
            // let index = self.node_indices[node];
            // centrality.insert(*node, eigenvector[self.graph[index].index()]);}
        }
    }
        return centrality;
    }

    pub fn analyze_mutual_connections(&self) -> Vec<(u32, u32, u32)> {
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
}

pub fn read_csv(filepath: &str) -> Result<Vec<TrustRelation>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut relations = Vec::new();

    // Iterate through each line of the CSV file
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        // Skip the header row
        if index == 0 {
            continue;
        }

        // Split the line by commas and parse the values
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let rater = parts[0].parse().unwrap_or(0);
            let ratee = parts[1].parse().unwrap_or(0);
            let trust_score = parts[2].parse().unwrap_or(0);

            // Add each parsed relation to the graph
            relations.push(TrustRelation {
                rater,
                ratee,
                trust_score,
            });
        }
    } 
    Ok(relations) 
}

fn main()->Result<()> {
    let relations = read_csv(r"C:\Users\nicho\projects\ds210final\DS210-Final-Project\BTCAlphaNet.csv");
    // Create and populate the TrustGraph
    let mut graph = TrustGraph::new();
    graph.add_relations(relations.expect("REASON"));
    let centrality = graph.compute_eigenvector_centrality(100, 1e-6);
    let _mutual_results = graph.analyze_mutual_connections();
    for (user, score) in centrality {
        println!("User {}: Centrality Score = {}", user, score);
    }
    println!("{:?}", graph);
    Ok(())
}
