use std::collections::HashSet;

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

    pub fn dfs(&self, start_vertex: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.dfs_recursive(start_vertex, &mut visited, &mut result);
        result
    }

    fn dfs_recursive(&self, vertex: usize, visited: &mut HashSet<usize>, result: &mut Vec<usize>) {
        visited.insert(vertex);
        result.push(vertex);

        for &neighbor in &self.adjacency_list[vertex] {
            if !visited.contains(&neighbor) {
                self.dfs_recursive(neighbor, visited, result);
            }
        }
    }
}