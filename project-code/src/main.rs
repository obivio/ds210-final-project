mod algorithm;

fn main() {
    let file_path = "src/amazon0302.txt"; // Path to your dataset file
    let n = 10000; // Size of the sample (can be adjusted)

    // Load the graph
    let graph = algorithm::load_graph(file_path);

    // Sample the top `n` nodes
    let sampled_graph = algorithm::sample_top_n_nodes(&graph, n);

    // Perform BFS starting from node 0 (or any other starting node)
    let bfs_result = algorithm::bfs(&sampled_graph, 0);

    // Output the BFS traversal order
    println!("BFS Traversal Order: {:?}", bfs_result);
}
