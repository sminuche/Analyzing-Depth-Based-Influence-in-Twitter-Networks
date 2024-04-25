use std::{
    collections::{HashMap, HashSet},
    fs,
};

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
}

fn load_dataset(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _filename = filename;
    let mut graph = Graph::new();
    let content = fs::read_to_string("higgs_social_network.edgelist")?;
    content.lines().for_each(|line| {
        let nodes: Vec<_> = line.trim().split(',').collect();
        if nodes.len() == 2 {
            graph.add_edge(nodes[0].to_string(), nodes[1].to_string());
        }
    });
    println!("{:?}", graph);
    Ok(())
}

fn main() {
    if let Err(err) = load_dataset("higgs_social_network.edgelist") {
        eprintln!("Error loading dataset: {}", err);
    }
}

