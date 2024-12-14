mod algorithm;
mod visualizations;
use std::io;

fn main() {
    let file_path = "src/amazon0302.txt"; 
    let sample_size = 15; 

    let graph = algorithm::load_graph(file_path);

    let sampled_nodes = algorithm::sample_connected_nodes(&graph, 0, sample_size);

    let sampled_nodes_vec: Vec<usize> = sampled_nodes.clone().into_iter().collect();

    let edge_list = algorithm::generate_edge_list(&graph, &sampled_nodes);

    let _bfs_result = algorithm::bfs(&graph, 0);


    let mut input = String::new();

    println!("Enter the start node:");
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let start_node: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    if !graph.contains_key(&start_node) {
        println!("Start node {} does not exist in the graph.", start_node);
        return;
    }

    input.clear();

    println!("Enter the end node:");
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let end_node: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    if !graph.contains_key(&end_node) {
        println!("End node {} does not exist in the graph.", end_node);
        return;
    }

    let path = algorithm::shortest_path(&graph, start_node, end_node);

    if !path.is_empty() {
        println!("Shortest path from {} to {}: {:?}", start_node, end_node, path);
    } else {
        println!("No path exists between {} and {}", start_node, end_node);
    }

    match visualizations::visualize_sampled_graph(&edge_list, &sampled_nodes_vec) {
        Ok(_) => println!("Graph visualization saved to 'graph.png'"),
        Err(e) => eprintln!("Error visualizing graph: {}", e),
    }
}
