use std::collections::{HashMap, HashSet};

trait IntoDot {
    fn to_dot(&self) -> String;
}

pub struct Node {
    id: String,
    label: String,
    color: String,
    attributes: String,
}

impl IntoDot for Node {
    fn to_dot(&self) -> String {
        let mut res: String = String::new();
        res.push_str(&format!(
            "\t\"n_{}\" [label=\"{}\" URL=\"/parameters/html/{}\" color=\"{}\" {}]\n", // todo remove the hardcoded '/parameters'
            self.id, self.label, self.id, self.color, self.attributes,
        ));
        res
    }
}

impl From<&Set> for Node {
    fn from(set: &Set) -> Node {
        let attributes = "shape=box".into();
        Node {
            id: set.id.to_string(),
            label: set.name.clone(),
            color: "#dddddd".into(),
            attributes,
        }
    }
}

pub struct Edge {
    pub from: String,
    pub to: String,
    pub label: String,
    pub attributes: String,
    pub url: String,
}

impl IntoDot for Edge {
    fn to_dot(&self) -> String {
        let mut res: String = String::new();
        res.push_str(&format!(
            "\t\"n_{}\" -> \"n_{}\" [label=\"{}\" {}]\n",
            self.from, self.to, self.label, self.attributes
        ));
        res
    }
}

impl From<&PreviewRelation> for Edge {
    fn from(prev: &PreviewRelation) -> Edge {
        let attributes = String::new();
        // attributes.append() ... todo
        Edge {
            from: prev.subset.id.to_string(),
            to: prev.superset.id.to_string(),
            label: "O".to_string(),
            attributes,
            url: prev.id.to_string(),
        }
    }
}

pub type SetColorCallback = dyn Fn(&Set) -> String;

pub struct Graph {
    pub name: String,
    pub color_fn: Option<Box<SetColorCallback>>,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new(name: &str, color_fn: Option<Box<SetColorCallback>>) -> Graph {
        Graph {
            name: name.into(),
            color_fn,
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, set: &Set) {
        let mut node: Node = Node::from(set);
        if let Some(f) = &self.color_fn {
            node.color = f(set);
        }
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn to_dot(&self) -> String {
        let test_set: HashSet<String> = HashSet::from_iter(self.nodes.iter().map(|n| n.id.clone()));
        for edge in &self.edges {
            assert!(test_set.contains(&edge.from));
            assert!(test_set.contains(&edge.to));
        }
        let mut dot = String::new();
        dot.push_str(&format!("digraph {} {{\n", self.name));
        dot.push_str("\tnode [color=lightblue2 style=filled]\n");
        dot.push_str("\tmargin=0.04 size=\"6,6\"\n");
        dot.push_str("\trankdir = BT\n");
        for node in &self.nodes {
            dot.push_str(&node.to_dot());
        }
        for edge in &self.edges {
            dot.push_str(&edge.to_dot());
        }
        // QoA8jA [label="acyclic chromatic number" URL="./QoA8jA" color="#c5d5e5" shape=box]
        // GNOiyB -> AGnF5Z_dist [label="●" URL="./bound_JngPPm_dist_to_AGnF5Z_dist" arrowhead=onormalnonenonecrow color="#000000" decorate=true fontcolor="#000000" lblstyle="above, sloped" penwidth=2.0 style=solid weight=100]
        dot.push_str("}\n");
        dot
    }
}

fn main() {
    let nodes = vec![Node {
        id: "dS6OgO".to_string(),
        label: "carving-width".to_string(),
        color: "#dddddd".into(),
        attributes: "label=\"carving-width\" URL=\"./dS6OgO\" color=\"#c5d5e5\" shape=box"
            .to_string(),
    }];
    fn color_fn(set: &Set) -> String {
        "gray".into()
    };
    let graph = Graph {
        name: "test".into(),
        color_fn: Some(Box::new(color_fn)),
        nodes,
        edges: vec![
            Edge {
                url: "qq".to_string(),
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
