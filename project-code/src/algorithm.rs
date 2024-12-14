use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_graph(file_path: &str) -> HashMap<usize, Vec<usize>> {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    let file = File::open(file_path).expect("Could not open file.");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line.");
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() != 2 {
            continue;
        }

        let from_node: usize = parts[0].parse().expect("Invalid node ID.");
        let to_node: usize = parts[1].parse().expect("Invalid node ID.");

        graph.entry(from_node).or_insert_with(Vec::new).push(to_node);
        graph.entry(to_node).or_insert_with(Vec::new).push(from_node);
    }

    graph
}

pub fn sample_connected_nodes(graph: &HashMap<usize, Vec<usize>>, start_node: usize, sample_size: usize) -> HashSet<usize> {
    let mut sampled_nodes = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start_node);
    sampled_nodes.insert(start_node);

    while let Some(node) = queue.pop_front() {
        if sampled_nodes.len() >= sample_size {
            break;
        }

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !sampled_nodes.contains(&neighbor) && sampled_nodes.len() < sample_size {
                    sampled_nodes.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    sampled_nodes
}

pub fn generate_edge_list(graph: &HashMap<usize, Vec<usize>>, sampled_nodes: &HashSet<usize>) -> Vec<(usize, usize)> {
    let mut edge_list = Vec::new();

    for (node, neighbors) in graph.iter() {
        if sampled_nodes.contains(&node) {
            for &neighbor in neighbors {
                if sampled_nodes.contains(&neighbor) {
                    edge_list.push((*node, neighbor));
                }
            }
        }
    }

    edge_list
}

pub fn bfs(graph: &HashMap<usize, Vec<usize>>, start_node: usize) -> Vec<usize> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut traversal_order: Vec<usize> = Vec::new();

    queue.push_back(start_node);
    visited.insert(start_node);

    // Print the starting node
    println!("Starting BFS from node: {}", start_node);

    while let Some(node) = queue.pop_front() {
        traversal_order.push(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    let limit = 20;
    let truncated_order: Vec<usize> = traversal_order.iter().take(limit).cloned().collect();
    let extra_count = if traversal_order.len() > limit { traversal_order.len() - limit } else { 0 };

    print!("Traversal order: ");
    println!("{}", truncated_order.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(", "));
    
    if extra_count > 0 {
        println!("... and {} more nodes.", extra_count);
    }

    traversal_order
}

pub fn shortest_path(graph: &HashMap<usize, Vec<usize>>, start_node: usize, end_node: usize) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut predecessors = HashMap::new();

    queue.push_back(start_node);
    visited.insert(start_node);

    while let Some(node) = queue.pop_front() {
        if node == end_node {
            break;
        }

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                    predecessors.insert(neighbor, node);
                }
            }
        }
    }

  
    let mut path = Vec::new();
    let mut current_node = end_node;

    while let Some(&predecessor) = predecessors.get(&current_node) {
        path.push(current_node);
        current_node = predecessor;

        if current_node == start_node {
            path.push(start_node);
            path.reverse();
            return path;
        }
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_load_graph() {
        let graph_data = vec![
            "1 2",
            "2 3",
            "3 1",
        ];

        let graph = load_graph_from_data(&graph_data);

        assert!(graph.get(&1).unwrap().contains(&2));
        assert!(graph.get(&1).unwrap().contains(&3));
        assert!(graph.get(&2).unwrap().contains(&1));
        assert!(graph.get(&2).unwrap().contains(&3));
        assert!(graph.get(&3).unwrap().contains(&1));
        assert!(graph.get(&3).unwrap().contains(&2));
    }

    fn load_graph_from_data(data: &[&str]) -> HashMap<usize, Vec<usize>> {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

        for line in data {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let from_node: usize = parts[0].parse().expect("Invalid node ID.");
            let to_node: usize = parts[1].parse().expect("Invalid node ID.");

            graph.entry(from_node).or_insert_with(Vec::new).push(to_node);
            graph.entry(to_node).or_insert_with(Vec::new).push(from_node); 
        }

        graph
    }
 
    #[test]
    fn test_sample_connected_nodes() {
        let graph: HashMap<usize, Vec<usize>> = vec![
            (1, vec![2, 3]),
            (2, vec![1, 3]),
            (3, vec![1, 2]),
        ].into_iter().collect();

        let sampled_nodes = sample_connected_nodes(&graph, 1, 2);

        assert_eq!(sampled_nodes.len(), 2);
        assert!(sampled_nodes.contains(&1));
        assert!(sampled_nodes.contains(&2));
    }


    #[test]
    fn test_generate_edge_list() {
        let graph: HashMap<usize, Vec<usize>> = vec![
            (1, vec![2, 3]),
            (2, vec![1, 3]),
            (3, vec![1, 2]),
        ].into_iter().collect();

        let sampled_nodes: HashSet<usize> = vec![1, 2].into_iter().collect();
        let edge_list = generate_edge_list(&graph, &sampled_nodes);

        assert!(edge_list.contains(&(1, 2)));
        assert!(edge_list.contains(&(2, 1)));
        assert_eq!(edge_list.len(), 2); 
    }


    #[test]
    fn test_bfs() {
        let graph = HashMap::from([
            (1, vec![2, 3]),
            (2, vec![1, 3]),
            (3, vec![1, 2]),
        ]);

        let bfs_result = bfs(&graph, 1);

        assert_eq!(bfs_result, vec![1, 2, 3]);
    }
}
