mod algorithm;
mod graphdata;

use algorithm::dijkstra;
use graphdata::preprocess_graph_with_sampling;

fn main() {
    // Define the dataset file path and sampling parameters
    let file_path = "src/amazon0302.txt"; // Update this with your dataset's actual path
    let sample_size = 10000; // Specify the number of nodes to sample

    // Preprocess the graph with sampling
    match preprocess_graph_with_sampling(file_path, sample_size) {
        Ok(graph) => {
            println!("Sampled graph loaded with {} nodes.", graph.len());

            // Define a starting node for the algorithm
            let start_node = 1; // Adjust this to a valid node in your graph

            // Run Dijkstra's Algorithm on the sampled graph
            let distances = dijkstra(&graph, start_node);

            // Display the shortest distances
            println!("Shortest distances from node {}:", start_node);
            for (node, distance) in &distances {
                println!("Node {}: Distance {}", node, distance);
            }
        }
        Err(e) => {
            eprintln!("Failed to preprocess the graph: {}", e);
        }
    }
}
