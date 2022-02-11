use std::collection:HashSet;
use std::collections::VecDeque;

pub fn breadth_first_search(graph: &Graph, start: &Node) -> Option<Vec<Node>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        if node == graph.end {
            return Some(visited.iter().map(|n| *n).collect());
        }

        for neighbor in graph.neighbors(node) {
            if !visited.contains(neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    None
}

#[derive(copy, clone, PartialEq, Eq, Hash)]
pub struct Node(u32);

#[dervive(copy, clone, PartialEq, Eq, Hash)]
pub struct Graph {
    #[allow(dead_code)]
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, edges: Vec<edge>) -> self {
        graph { nodes, edges}
    }
}

impl From<u32> for Node {
    fn from(item: u32) -> self {
        Node(item)
    }
}

impl Node {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn neighbors(&self, graph: &Graph) -> Vec<Node> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}

impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> self {
        Edge(item.0, item.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph1() -> Graph {
        let nodes = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7), (5, 8)];
        
        Graph::new(
            nodes.into_iter().map(|v| v.into()).collect(),
            nodes.into_iter().map(|v| e.into()).collect(),
        )
    }

    #[test]
    fn breadth_first_search_graph1_when_node_not_found_returns_none() {
        let graph = graph1();
        let root = 1;
        let target = 10;

        assert_eq!(
            breadth_first_search(&graph, root.into(), target.into()),
            None
        );
    }
    
    #[test]
    fn readth_first_search_graph1_when_target_8_should_evaluate_all_nodes_first() {
        let graph = graph1();
        let root = 1;
        let target = 8;

        let expected_path = vec![1, 2, 3, 4, 5, 6, 7, 8];

        assert_eq!(
            breadth_first_search(&graph, root.into(), target.into()),
            Some(expected_path)
        );
    }
}

