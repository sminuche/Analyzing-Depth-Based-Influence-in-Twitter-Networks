mod data_processing;
mod graph;

use data_processing::load_dataset;

#[cfg(test)]
mod tests {
    use crate::graph::Graph;
    use std::collections::HashSet;

    #[test]
    fn test_breadth_first_search() {
        let mut graph = Graph::new();
        graph.add_edge("A".to_string(), "B".to_string());
        graph.add_edge("A".to_string(), "C".to_string());
        graph.add_edge("B".to_string(), "D".to_string());
        graph.add_edge("C".to_string(), "E".to_string());
        graph.add_edge("D".to_string(), "E".to_string());

        let result = graph.breadth_first_search("A", 2);
        let expected: HashSet<String> = vec!["A", "B", "C", "D", "E"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_breadth_first_search_depths() {
        let mut graph = Graph::new();
        graph.add_edge("A".to_string(), "B".to_string());
        graph.add_edge("A".to_string(), "C".to_string());
        graph.add_edge("B".to_string(), "D".to_string());
        graph.add_edge("C".to_string(), "E".to_string());
        graph.add_edge("D".to_string(), "E".to_string());

        let result_depth_1 = graph.breadth_first_search("A", 1);
        let expected_depth_1: HashSet<String> = vec!["A", "B", "C"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(result_depth_1, expected_depth_1);

        let result_depth_2 = graph.breadth_first_search("A", 2);
        let expected_depth_2: HashSet<String> = vec!["A", "B", "C", "D", "E"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(result_depth_2, expected_depth_2);
    }

    #[test]
    fn test_edge_cases() {
        let empty_graph = Graph::new();
        let empty_result = empty_graph.breadth_first_search("A", 1);
        let empty_expected: HashSet<String> =
            vec!["A"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!(empty_result, empty_expected);

        let single_node_graph = Graph::new();
        let single_node_result = single_node_graph.breadth_first_search("A", 1);
        let expected_single_node: HashSet<String> =
            vec!["A"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!(single_node_result, expected_single_node);

        let mut cyclic_graph = Graph::new();
        cyclic_graph.add_edge("A".to_string(), "B".to_string());
        cyclic_graph.add_edge("B".to_string(), "A".to_string());
        let cyclic_result = cyclic_graph.breadth_first_search("A", 2);
        let expected_cyclic: HashSet<String> =
            vec!["A", "B"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!(cyclic_result, expected_cyclic);
    }
}

fn main() {
    let filename = "higgs_social_network.edgelist";

    if let Err(err) = load_dataset(filename) {
        eprintln!("Error loading dataset: {}", err);
    }


}