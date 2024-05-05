use rand::prelude::SliceRandom;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Graph {
    pub adjacency_list: HashMap<String, HashSet<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, node1: String, node2: String) {
        self.add_single_edge(node1.clone(), node2.clone());
        self.add_single_edge(node2, node1);
    }

    fn add_single_edge(&mut self, from: String, to: String) {
        self.adjacency_list
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    pub fn breadth_first_search(&self, start_node: &str, depth: usize) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut queue = Vec::new();
        queue.push((start_node.to_string(), 0));

        while let Some((current_node, current_depth)) = queue.pop() {
            if visited.contains(&current_node) || current_depth >= depth {
                continue;
            }
            visited.insert(current_node.clone());

            if let Some(neighbors) = self.adjacency_list.get(&current_node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        queue.push((neighbor.clone(), current_depth + 1));
                    }
                }
            }
        }
        visited
    }

    pub fn sample_random_nodes(&self, num_nodes: usize) -> Vec<String> {
        let keys: Vec<_> = self.adjacency_list.keys().cloned().collect();
        let mut rng = rand::thread_rng();
        let mut shuffled_keys = keys
            .choose_multiple(&mut rng, num_nodes)
            .cloned()
            .collect::<Vec<String>>();
        shuffled_keys.shuffle(&mut rng);
        shuffled_keys
    }
}
