use std::{collections::{HashMap, HashSet}, fs};
use rand::prelude::SliceRandom;

#[derive(Debug)]
struct Graph {
    adjacency_list: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Self {
        Self { adjacency_list: HashMap::new() }
    }

    fn add_edge(&mut self, node1: String, node2: String) {
        self.adjacency_list.entry(node1.clone()).or_insert_with(HashSet::new).insert(node2.clone());
        self.adjacency_list.entry(node2).or_insert_with(HashSet::new).insert(node1);
    }

    fn breadth_first_search(&self, node: &str, depth: usize) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut queue = Vec::new();
        queue.push((node.to_string(), 0));
    
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

    fn sample_random_nodes(&self, num_nodes: usize) -> Vec<String> {
        let mut rng = rand::thread_rng();
        let keys: Vec<_> = self.adjacency_list.keys().cloned().collect();
        let mut shuffled_keys: Vec<String> = keys.choose_multiple(&mut rng, num_nodes).cloned().collect();
        shuffled_keys.shuffle(&mut rng);
        shuffled_keys
    }
}

fn load_dataset(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    
    let mut distance_map: HashMap<&String, Vec<f64>> = HashMap::new();
    let random_nodes = graph.sample_random_nodes(batch_size);

    for depth in 2..=4 {
        //let (mut total_overlap, mut total_followers) = (0, 0);
        println!("{:?}", distance_map);

        for node in &random_nodes {
            //let direct_friends = graph.adjacency_list.get(&node).unwrap_or(&HashSet::new()).clone();
            let friends_of_friends = graph.breadth_first_search(&node, depth);
            let proportion =  friends_of_friends.len() as f64/total_nodes as f64;
            println!("{}", proportion);
            if let Some(proportions) = distance_map.get_mut(node) {
                proportions.push(proportion);
                
            } else {
                let initialized_proportions = vec![proportion];
                distance_map.insert(node, initialized_proportions);

            }


            //println!("{:?}",friends_of_friends);

            //total_overlap += direct_friends.intersection(&friends_of_friends).count();
            //println!("total overlap: {}", total_overlap);
            //total_followers += friends_of_friends.len();
            //println!("total followers: {}", total_followers);

        }

    //let average_overlap = if total_followers > 0 { total_overlap as f64 / total_followers as f64 } else { 0.0 };
    //println!("Average overlap at depth {}: {:.2}", depth, average_overlap);
    }

    for(key, value) in &distance_map {
        println!("Node: {}, Proportions: {:?}", key, value)
    }
    Ok(())
}


fn main() {
    if let Err(err) = load_dataset("higgs_social_network.edgelist") {
        eprintln!("Error loading dataset: {}", err);
    }
}
