use std::collections::HashMap;

use crate::{data::Set, raw::RawRelation};

pub struct Node {
    label: String,
    attributes: String,
}

impl Into<Node> for &Set {
    fn into(self) -> Node {
        let mut attributes = String::new();
        Node {
            label: self.name.clone(),
            attributes,
        }
    }
}

pub struct Edge {
    from: String,
    to: String,
    label: String,
    attributes: String,
}

impl Into<Edge> for &RawRelation {
    fn into(self) -> Edge {
        let attributes = String::new();
        // attributes.append() ... todo
        Edge {
            from: self.subset.id.clone(),
            to: self.superset.id.clone(),
            label: "O".to_string(),
            attributes,
        }
    }
}

pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub edges: Vec<Edge>,
}

impl Graph {

    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, set: &Set) {
        self.nodes.insert(set.id.clone(), set.into());
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("digraph unix {\n");
        dot.push_str("  node [color=lightblue2 style=filled]\n");
        dot.push_str("  margin=0.04 size=\"6,6\"\n");
        for (node_id, node) in &self.nodes {
            dot.push_str(&format!("  n_{} [{}]\n", node_id, node.attributes));
        }
        for edge in &self.edges {
            dot.push_str(&format!(
                "  n_{} -> n_{} [label=\"{}\" {}]\n",
                edge.from, edge.to, edge.label, edge.attributes
            ));
        }
        // QoA8jA [label="acyclic chromatic number" URL="./QoA8jA" color="#c5d5e5" shape=box]
        // GNOiyB -> AGnF5Z_dist [label="●" URL="./bound_JngPPm_dist_to_AGnF5Z_dist" arrowhead=onormalnonenonecrow color="#000000" decorate=true fontcolor="#000000" lblstyle="above, sloped" penwidth=2.0 style=solid weight=100]
        dot.push_str("}\n");
        dot
    }

}

fn main() {
    let mut nodes = HashMap::new();
    nodes.insert(
        "dS6OgO".to_string(),
        Node {
            label: "carving-width".to_string(),
            attributes: "label=\"carving-width\" URL=\"./dS6OgO\" color=\"#c5d5e5\" shape=box".to_string(),
        },
    );

    let graph = Graph {
        nodes,
        edges: vec![
            Edge {
                from: "GNOiyB".to_string(),
                to: "AGnF5Z_dist".to_string(),
                label: "●".to_string(),
                attributes: "URL=\"./bound_JngPPm_dist_to_AGnF5Z_dist\" arrowhead=onormalnonenonecrow color=\"#000000\" decorate=true fontcolor=\"#000000\" lblstyle=\"above, sloped\" penwidth=2.0 style=solid weight=100".to_string(),
            },
        ],
    };

    let dot_string = graph.to_dot();
    println!("{}", dot_string);
}
