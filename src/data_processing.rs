use crate::graph::Graph;
use std::{collections::HashMap, error::Error, fs};

pub fn print_best_nodes(node_vec: Vec<(usize, String, f64)>) {
    for (depth, node, prop) in node_vec {
        println!(
            "For depth {}, the most popular node is {}, covering {:.2}% of all users",
            depth,
            node,
            prop * 100.0
        );
    }
}

pub fn load_dataset(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut graph = Graph::new();
    let content = fs::read_to_string(filename)?;

    for line in content.lines() {
        let nodes: Vec<_> = line.split_whitespace().collect();
        if nodes.len() == 2 {
            graph.add_edge(nodes[0].to_string(), nodes[1].to_string());
        }
    }

    let total_nodes = graph.adjacency_list.len();
    let batch_size = 100;

    let distance_map = compute_distance_map(&graph, total_nodes, batch_size);

    let max_vector = find_max_proportions(&distance_map);

    print_best_nodes(max_vector);
    Ok(())
}

pub fn compute_distance_map(
    graph: &Graph,
    total_nodes: usize,
    batch_size: usize,
) -> HashMap<String, Vec<f64>> {
    let mut distance_map: HashMap<String, Vec<f64>> = HashMap::new();
    let random_nodes = graph.sample_random_nodes(batch_size);

    for depth in 1..=6 {
        for node in &random_nodes {
            let friends_of_friends = graph.breadth_first_search(&node, depth);
            let proportion = friends_of_friends.len() as f64 / total_nodes as f64;

            distance_map
                .entry(node.clone())
                .or_insert_with(Vec::new)
                .push(proportion);
        }
    }
    distance_map
}

pub fn find_max_proportions(distance_map: &HashMap<String, Vec<f64>>) -> Vec<(usize, String, f64)> {
    let mut max_vector = Vec::new();

    for depth in 1..=6 {
        let mut max_proportion = 0.0;
        let mut max_node = String::new();

        for (node, proportions) in distance_map {
            if let Some(&proportion) = proportions.get(depth - 1) {
                if proportion > max_proportion {
                    max_proportion = proportion;
                    max_node = node.clone();
                }
            }
        }

        max_vector.push((depth, max_node, max_proportion));
    }

    max_vector
}
