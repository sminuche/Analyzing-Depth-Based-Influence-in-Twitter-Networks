use std::{collections::{HashMap, HashSet, VecDeque}, fs, time::SystemTime};
use rand::seq::SliceRandom;

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
        let mut queue = VecDeque::new();
        queue.push_back((node.to_string(), 0));
    
        while let Some((current_node, current_depth)) = queue.pop_front() {
            if visited.contains(&current_node) || current_depth >= depth {
                continue;
            }
            visited.insert(current_node.clone());
    
            if let Some(neighbors) = self.adjacency_list.get(&current_node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        queue.push_back((neighbor.clone(), current_depth + 1));
                    }
                }
            }
        }
        visited
    }

    fn sample_random_nodes(&self, num_nodes: usize) -> Vec<String> {
        let mut rng = rand::thread_rng();
        let keys: Vec<_> = self.adjacency_list.keys().cloned().collect();
        let mut shuffled_keys = keys.clone();
        shuffled_keys.shuffle(&mut rng);
        shuffled_keys.into_iter().take(num_nodes).collect()
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

    let total_nodes = graph.adjacency_list.len().min(1_000); 
    let batch_size = 100;
    let num_batches = (total_nodes as f64 / batch_size as f64).ceil() as usize;

    let (mut total_overlap, mut total_followers) = (0, 0);

    for _ in 0..num_batches {
        let random_nodes = graph.sample_random_nodes(batch_size);
        for node in random_nodes {
            let direct_friends = graph.adjacency_list.get(&node).unwrap_or(&HashSet::new()).clone();
            let friends_of_friends = graph.breadth_first_search(&node, 2);

            total_overlap += direct_friends.intersection(&friends_of_friends).count();
            total_followers += friends_of_friends.len();
        }
    }

    let average_overlap = if total_followers > 0 { total_overlap as f64 / total_followers as f64 } else { 0.0 };
    println!("Average overlap: {:.2}", average_overlap);

    Ok(())
}


fn main() {
    let start_time = SystemTime::now();
    if let Err(err) = load_dataset("higgs_social_network.edgelist") {
        eprintln!("Error loading dataset: {}", err);
    }
    println!("Total time taken: {:?}", start_time.elapsed().unwrap());
}
