use std::collections::HashSet;
use rand::seq::IteratorRandom;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

/// Represents an edge in the graph
pub type Edge = (usize, usize);

/// Load the graph from a text file
pub fn load_graph(file_path: &str) -> io::Result<Vec<Edge>> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let u = parts[0].parse::<usize>().unwrap();
            let v = parts[1].parse::<usize>().unwrap();
            edges.push((u, v));
        }
    }
    Ok(edges)
}

/// Randomly sample nodes and extract their edges
pub fn sample_graph(edges: &[Edge], num_nodes: usize) -> Vec<Edge> {
    let mut rng = rand::thread_rng();

    // Collect unique nodes
    let unique_nodes: HashSet<usize> = edges.iter().flat_map(|(u, v)| vec![*u, *v]).collect();
    let sampled_nodes: HashSet<usize> = unique_nodes
        .iter()
        .cloned()
        .choose_multiple(&mut rng, num_nodes)
        .into_iter()
        .collect();

    // Filter edges where both nodes are in the sampled set
    edges
        .iter()
        .filter(|(u, v)| sampled_nodes.contains(u) && sampled_nodes.contains(v))
        .cloned()
        .collect()
}

/// Save the subset of edges to a new file
pub fn save_graph(edges: &[Edge], output_path: &str) -> io::Result<()> {
    let mut file = File::create(output_path)?;
    for (u, v) in edges {
        writeln!(file, "{} {}", u, v)?;
    }
    Ok(())
}
