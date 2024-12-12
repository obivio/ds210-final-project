use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Function to load the dataset from a file and return a graph representation
/// as an adjacency list.
pub fn load_graph(file_path: &str) -> HashMap<usize, Vec<usize>> {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    let file = File::open(file_path).expect("Could not open file.");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line.");
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // Skip the header lines with meta-information
        if parts.len() != 2 {
            continue;
        }

        let from_node: usize = parts[0].parse().expect("Invalid node ID.");
        let to_node: usize = parts[1].parse().expect("Invalid node ID.");

        graph.entry(from_node).or_insert_with(Vec::new).push(to_node);
        graph.entry(to_node).or_insert_with(Vec::new).push(from_node); // Undirected edges
    }

    graph
}

/// Function to sample the top `n` nodes based on their node ID (or any other criteria)
/// and return a subgraph as an adjacency list.
pub fn sample_top_n_nodes(graph: &HashMap<usize, Vec<usize>>, n: usize) -> HashMap<usize, Vec<usize>> {
    let mut sampled_graph: HashMap<usize, Vec<usize>> = HashMap::new();

    // Make sure node 0 is included in the sample
    let mut top_n_nodes: Vec<usize> = graph.keys().cloned().take(n).collect();

    if !top_n_nodes.contains(&0) {
        top_n_nodes.push(0); // Add node 0 if it's not already included
    }

    for node in top_n_nodes {
        if let Some(neighbors) = graph.get(&node) {
            let mut sorted_neighbors = neighbors.clone(); // Clone the neighbors to modify them
            sorted_neighbors.sort(); // Sort neighbors in numerical order
            sampled_graph.insert(node, sorted_neighbors);
        }
    }

    // Debugging: Print sampled graph in numerical order
    let mut sorted_nodes: Vec<usize> = sampled_graph.keys().cloned().collect();
    sorted_nodes.sort(); // Sort nodes in numerical order

    for node in sorted_nodes {
        if let Some(neighbors) = sampled_graph.get(&node) {
            println!("Node {} has neighbors: {:?}", node, neighbors);
        }
    }

    sampled_graph
}



/// BFS function to traverse the graph from a starting node and return the traversal order.
pub fn bfs(graph: &HashMap<usize, Vec<usize>>, start_node: usize) -> Vec<usize> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut traversal_order: Vec<usize> = Vec::new();

    queue.push_back(start_node);
    visited.insert(start_node);

    // Debugging: Print initial state
    println!("Starting BFS from node: {}", start_node);

    while let Some(node) = queue.pop_front() {
        traversal_order.push(node);

        if let Some(neighbors) = graph.get(&node) {
            // Debugging: print the neighbors of the current node
            println!("Node {} has neighbors: {:?}", node, neighbors);

            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                    // Debugging: print state after adding a neighbor
                    println!("Added node {} to the queue", neighbor);
                }
            }
        }
    }

    traversal_order
}


