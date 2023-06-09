use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Graph {
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    // Constructor that returns a new, empty graph  
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    // Method to add an edge between two nodes in the graph
    pub fn add_edge(&mut self, source: usize, target: usize) {
        self.edges.entry(source).or_insert(Vec::new()).push(target);
        self.edges.entry(target).or_insert(Vec::new()).push(source);
    }

    // Method to get the degree of a node in the graph
    pub fn degree(&self, node: usize) -> usize {
        // Get the vector of neighbors associated with the given node and return its length, or 0 if the node is not in the graph
        self.edges.get(&node).map_or(0, |neighbors| neighbors.len())
    }

    // Method to get the set of nodes that are neighbors of a node at a distance of 2 in the graph
    pub fn neighbors_at_distance_2(&self, node: usize) -> HashSet<usize> {
        let mut neighbors = HashSet::new();
    
        // Get the vector of neighbors associated with the given node
        if let Some(adjacent_nodes) = self.edges.get(&node) {
            // For each adjacent node, get its vector of neighbors and add them to the HashSet if they are not equal to the given node
            for &adjacent_node in adjacent_nodes {
                if let Some(second_order_nodes) = self.edges.get(&adjacent_node) {
                    neighbors.extend(second_order_nodes.iter().filter(|&&n| n != node));
                }
            }
        }
        neighbors
    }

    // Method to get a vector of all nodes in the graph
    pub fn nodes(&self) -> Vec<usize> {
        self.edges.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        assert_eq!(graph.edges.get(&0), Some(&vec![1]));
        assert_eq!(graph.edges.get(&1), Some(&vec![0, 2]));
        assert_eq!(graph.edges.get(&2), Some(&vec![1, 3]));
        assert_eq!(graph.edges.get(&3), Some(&vec![2]));
    }

    #[test]
    fn test_degree() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        assert_eq!(graph.degree(0), 1);
        assert_eq!(graph.degree(1), 2);
        assert_eq!(graph.degree(2), 2);
        assert_eq!(graph.degree(3), 1);
    }

    #[test]
    fn test_neighbors_at_distance_2() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);

        let neighbors = graph.neighbors_at_distance_2(0);
        let expected_neighbors: HashSet<usize> = [2, 3].iter().cloned().collect();
        assert_eq!(neighbors, expected_neighbors);
    }

    #[test]
    fn test_nodes() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let nodes = graph.nodes();
        let expected_nodes: HashSet<usize> = [0, 1, 2].iter().cloned().collect();
        assert_eq!(nodes.iter().cloned().collect::<HashSet<usize>>(), expected_nodes);
    }
}
