mod csv;
mod centrality;
mod relationships;
mod tests;
use std::io::Result;
use crate::csv::TrustGraph;
use crate::csv::read_csv;
use crate::relationships::MutualConnectionAnalysis;
use crate::centrality::EigenvectorCentrality;

fn main()->Result<()> {
    let relations = read_csv(r"C:\Users\nicho\projects\ds210final\DS210-Final-Project\BTCAlphaNet.csv");
    // Create and populate the TrustGraph
    let mut graph = TrustGraph::new();
    graph.add_relations(relations.expect("REASON"));
    let centrality = graph.compute_eigenvector_centrality(100, 1e-6);
    for (user, score) in centrality {
        println!("User {}: Centrality Score = {}", user, score);
    }
    let _mutual_results = graph.analyze_mutual_connections();
    let mutual_scores = graph.find_mutual_trust_score();
    for ((node1, node2), score) in mutual_scores {
        println!("Mutual score between {} and {} is {}", node1, node2, score);
    }
    println!("{:?}", graph);
    Ok(())
}
