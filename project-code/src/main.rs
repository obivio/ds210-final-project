// Main function
mod algorithm;
mod visualizations;

// Main function
fn main() {
    let file_path = "src/amazon0302.txt"; // Path to your dataset file
    let sample_size = 15; // Number of nodes to sample (adjust as needed)

    // Load the graph
    let graph = algorithm::load_graph(file_path);

    // Sample connected nodes starting from node 0
    let sampled_nodes = algorithm::sample_connected_nodes(&graph, 0, sample_size);

    // Clone the sampled_nodes for visualization
    let sampled_nodes_vec: Vec<usize> = sampled_nodes.clone().into_iter().collect();

    // Generate the edge list for the sampled nodes (using the original sampled_nodes)
    let edge_list = algorithm::generate_edge_list(&graph, &sampled_nodes);

    // Perform BFS starting from node 0 (or any other starting node)
    let _bfs_result = algorithm::bfs(&graph, 0);

    // Output the BFS traversal order
    // The BFS function will print a concise summary of the traversal

    // Visualize the sampled graph
    match visualizations::visualize_sampled_graph(&edge_list, &sampled_nodes_vec) {
        Ok(_) => println!("Graph visualization saved to 'graph.png'"),
        Err(e) => eprintln!("Error visualizing graph: {}", e),
    }
}
