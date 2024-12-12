use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

/// Represents a graph as an adjacency list
pub type Graph = HashMap<usize, Vec<usize>>;

/// Dijkstra's Algorithm for an unweighted graph
pub fn dijkstra(graph: &Graph, start: usize) -> HashMap<usize, usize> {
    let mut distances: HashMap<usize, usize> = HashMap::new();
    let mut priority_queue: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();

    // Initialize distances with infinity
    for &node in graph.keys() {
        distances.insert(node, usize::MAX);
    }

    // Ensure all neighbors are also initialized
    for neighbors in graph.values() {
        for &neighbor in neighbors {
            distances.entry(neighbor).or_insert(usize::MAX);
        }
    }

    distances.insert(start, 0);

    // Add the starting node to the priority queue
    priority_queue.push(Reverse((0, start))); // (distance, node)

    while let Some(Reverse((current_distance, current_node))) = priority_queue.pop() {
        // If this distance is already worse, skip
        if let Some(&existing_distance) = distances.get(&current_node) {
            if current_distance > existing_distance {
                continue;
            }
        } else {
            // If the node isn't in distances for some reason, skip it
            continue;
        }

        // Explore neighbors
        if let Some(neighbors) = graph.get(&current_node) {
            for &neighbor in neighbors {
                let new_distance = current_distance + 1; // Unweighted graph, so all edge weights are 1

                // If a shorter path to the neighbor is found
                if let Some(&current_neighbor_distance) = distances.get(&neighbor) {
                    if new_distance < current_neighbor_distance {
                        distances.insert(neighbor, new_distance);
                        priority_queue.push(Reverse((new_distance, neighbor)));
                    }
                }
            }
        }
    }

    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let mut graph: Graph = HashMap::new();
        graph.insert(1, vec![2, 3]); // Node 1 connects to 2 and 3
        graph.insert(2, vec![3, 4]); // Node 2 connects to 3 and 4
        graph.insert(3, vec![4]);    // Node 3 connects to 4
        graph.insert(4, vec![]);    // Node 4 has no outgoing edges

        let distances = dijkstra(&graph, 1);

        assert_eq!(*distances.get(&1).unwrap(), 0); // Distance to self
        assert_eq!(*distances.get(&2).unwrap(), 1); // Distance to 2
        assert_eq!(*distances.get(&3).unwrap(), 1); // Distance to 3
        assert_eq!(*distances.get(&4).unwrap(), 2); // Distance to 4
    }

    #[test]
    fn test_dijkstra_with_missing_node() {
        let mut graph: Graph = HashMap::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![]);

        let distances = dijkstra(&graph, 4); // Start node not in graph
        assert!(distances.is_empty() || distances.get(&4).is_none());
    }
}
