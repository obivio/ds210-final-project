use rand::seq::IteratorRandom;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn preprocess_graph_with_sampling(
    file_path: &str,
    sample_size: usize,
) -> Result<HashMap<usize, Vec<usize>>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut is_header = true; // Flag to skip header lines

    for line in reader.lines() {
        let line = line?; // Read the line, handle IO errors

        // Skip header lines
        if is_header && line.starts_with('#') {
            continue;
        }
        is_header = false; // Stop treating lines as headers once the first non-comment is encountered

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            eprintln!("Skipping malformed line: {}", line);
            continue;
        }

        // Parse nodes, log errors without panicking
        let source = match parts[0].parse::<usize>() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Invalid source node in line '{}': {}", line, e);
                continue;
            }
        };
        let target = match parts[1].parse::<usize>() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Invalid target node in line '{}': {}", line, e);
                continue;
            }
        };

        graph.entry(source).or_insert_with(Vec::new).push(target);
        graph.entry(target).or_insert_with(Vec::new); // Ensure target node is in the graph
    }

    // Randomly sample nodes
    let sampled_graph: HashMap<usize, Vec<usize>> = graph
        .iter()
        .choose_multiple(&mut rand::thread_rng(), sample_size)
        .into_iter()
        .map(|(&k, v)| (k, v.clone()))
        .collect();

    Ok(sampled_graph)
}
