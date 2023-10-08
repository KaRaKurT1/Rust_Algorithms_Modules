use std::collections::{HashSet, VecDeque};

pub struct Graph {
    pub vertices: usize,
    adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            adjacency_list: vec![vec![]; vertices],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.adjacency_list[from].push(to);
    }

    pub fn bfs(&self, start_vertex: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        visited.insert(start_vertex);
        queue.push_back(start_vertex);

        while !queue.is_empty() {
            let current_vertex = queue.pop_front().unwrap();
            result.push(current_vertex);

            for &neighbor in &self.adjacency_list[current_vertex] {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        result
    }
}