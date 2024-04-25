use std::{
    collections::{HashMap, HashSet},
    fs,
    time::SystemTime
};

use rand::seq::SliceRandom;

#[derive(Debug)]
struct Graph {
    adjacency_list: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, node1: String, node2: String) {
        self.adjacency_list
            .entry(node1.clone())
            .or_insert_with(HashSet::new)
            .insert(node2.clone());
        self.adjacency_list
            .entry(node2)
            .or_insert_with(HashSet::new)
            .insert(node1);
    }

    fn calculate_degrees(&self) -> HashMap<String, usize> {
        let mut degrees_map: HashMap<String, usize> = HashMap::new();
    
        for (node, neighbors) in &self.adjacency_list {
            let mut degree_count = neighbors.len();
    
            for neighbor in neighbors {
                if let Some(neighbor_neighbors) = self.adjacency_list.get(neighbor) {
                    degree_count += neighbor_neighbors.len() - 1;
                }
            }
    
            degrees_map.insert(node.clone(), degree_count);
        }
    
        degrees_map
    }
    
    fn find_interconnected_nodes(&self) -> HashSet<String> {
        let mut interconnected_nodes = HashSet::new();
    
        for (node, _) in &self.adjacency_list {
            let mut visited = HashSet::new();
            let mut to_visit = vec![node.clone()];
    
            while let Some(current_node) = to_visit.pop() {
                if visited.contains(&current_node) {
                    continue;
                }
    
                visited.insert(current_node.clone());
                if let Some(neighbors) = self.adjacency_list.get(&current_node) {
                    for neighbor in neighbors {
                        if !visited.contains(neighbor) {
                            to_visit.push(neighbor.clone());
                            interconnected_nodes.insert(neighbor.clone());
                        }
                    }
                }
            }
        }
    
        interconnected_nodes
    }

}


fn load_dataset(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _filename = filename;
    let mut graph = Graph::new();
    let content = fs::read_to_string("higgs_social_network.edgelist")?;

    let mut rng = rand::thread_rng();
    let nodes: Vec<&str> = content.lines().collect();
    let mut batches = nodes.chunks(1000).collect::<Vec<_>>();
    batches.shuffle(&mut rng);

    for batch in batches.iter().take(20) {
        for &line_item in batch.iter() {
            let nodes: Vec<&str> = line_item.trim().split_whitespace().collect();
            if nodes.len() == 2 {
                graph.add_edge(nodes[0].to_string(), nodes[1].to_string());
            }
        }

        let degrees_map = graph.calculate_degrees();
        let interconnected_nodes = graph.find_interconnected_nodes();

        println!("Degrees for batch: {:?}", degrees_map);
        println!("Interconnected Nodes for batch: {:?}", interconnected_nodes);

        graph = Graph::new();
    }

    Ok(())
}

fn main() {
    
    let start_time = SystemTime::now();
    if let Err(err) = load_dataset("higgs_social_network.edgelist") {
        eprintln!("Error loading dataset: {}", err);
    }

    println!("Total time taken: {:?}", start_time.elapsed().unwrap());
}
        


