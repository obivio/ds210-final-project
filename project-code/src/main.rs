mod graph_preprocessing;

use graph_preprocessing::{load_graph, sample_graph, save_graph};

fn main() -> std::io::Result<()> {
    let input_file = "src/amazon0302.txt"; // Path to the full dataset
    let output_file = "./subset_graph.txt"; // Path for the preprocessed subset
    let num_nodes = 10000; // Number of nodes to sample

    // Load the graph
    let edges = load_graph(input_file)?;

    // Sample a subset
    let sampled_edges = sample_graph(&edges, num_nodes);

    // Save the subset
    save_graph(&sampled_edges, output_file)?;

    println!("Subset of the graph saved to {}", output_file);

    Ok(())
}
