use std::collections::{HashMap, VecDeque};

pub struct Vertex<VertexData> {
    pub id: String,
    pub data: VertexData,
}

pub struct Edge<EdgeData> {
    pub from: String,
    pub to: String,
    pub data: EdgeData,
}

pub struct DiGraph<VertexData, EdgeData> {
    pub vertices: HashMap<String, Vertex<VertexData>>,
    pub edges: HashMap<(String, String), Edge<EdgeData>>,
    out_edges: HashMap<String, Vec<String>>,
    in_edges: HashMap<String, Vec<String>>,
}

impl<VertexData, EdgeData> DiGraph<VertexData, EdgeData> {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            out_edges: HashMap::new(),
            in_edges: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, id: String, vertex: Vertex<VertexData>) {
        self.vertices.insert(id, vertex);
    }

    pub fn add_edge(&mut self, edge: Edge<EdgeData>) {
        self.out_edges
            .entry(edge.from.clone())
            .or_default()
            .push(edge.to.clone());
        self.in_edges
            .entry(edge.to.clone())
            .or_default()
            .push(edge.from.clone());
        self.edges
            .insert((edge.from.clone(), edge.to.clone()), edge);
    }

    pub fn bfs_get_distance(&self, set: String) -> HashMap<String, usize> {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        visited.insert(set.clone(), 0);
        queue.push_back((set, 0));
        while let Some((current_vertex, current_distance)) = queue.pop_front() {
            for edge_set in [
                &self.out_edges[&current_vertex],
                &self.in_edges[&current_vertex],
            ] {
                for other_endpoint in edge_set {
                    if !visited.contains_key(other_endpoint) {
                        let new_distance = current_distance + 1;
                        visited.insert(other_endpoint.clone(), new_distance);
                        queue.push_back((other_endpoint.clone(), new_distance));
                    }
                }
            }
        }
        visited
    }
}

impl<VertexData, EdgeData> Default for DiGraph<VertexData, EdgeData> {
    fn default() -> Self {
        Self::new()
    }
}
