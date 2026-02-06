use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::data::data::Parameter;
use crate::data::digraph::{DiGraph, Edge, Vertex};
use crate::general::file;
use crate::output::color::Color;

trait IntoDot {
    fn to_dot(&self) -> String;
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum DotVertexAttribute {
    Label(String),
    Color(Color),
    Url(String),
    Shape(NodeShape),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum NodeShape {
    Box,
}

impl From<&DotVertexAttribute> for String {
    fn from(attr: &DotVertexAttribute) -> String {
        match attr {
            DotVertexAttribute::Label(str) => format!("label=\"{}\"", str),
            DotVertexAttribute::Color(color) => format!("color=\"{}\"", color.hex()),
            DotVertexAttribute::Url(url) => format!("URL=\"/parameters/html/{}\"", url),
            DotVertexAttribute::Shape(shape) => format!(
                "shape=\"{}\"",
                match shape {
                    NodeShape::Box => "box",
                }
            ),
        }
    }
}

fn edges_to_string(attributes: &HashSet<DotEdgeAttribute>) -> String {
    attributes
        .iter()
        .fold(String::new(), |k, x| k + &String::from(x))
}

fn nodes_to_string(attributes: &HashSet<DotVertexAttribute>) -> String {
    attributes
        .iter()
        .fold(String::new(), |k, x| k + &String::from(x))
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum DotEdgeAttribute {
    Label(String),
    Url(String),
    Style(String),
    Color(Color),
    Weight(u32),
    PenWidth(u32),
    Uncategorized(String),
}

impl From<&DotEdgeAttribute> for String {
    fn from(attr: &DotEdgeAttribute) -> String {
        match attr {
            DotEdgeAttribute::Color(color) => format!("color=\"{}\"", color.hex()),
            DotEdgeAttribute::Label(str) => format!("label=\"{}\"", str),
            DotEdgeAttribute::PenWidth(width) => {
                format!("penwidth=\"{}.{}\"", width / 100, width % 100)
            }
            DotEdgeAttribute::Style(style) => format!("style=\"{}\"", style),
            DotEdgeAttribute::Url(url) => format!("URL=\"/parameters/html/{}\"", url), // todo remove the hardcoded '/parameters'
            DotEdgeAttribute::Uncategorized(other) => other.clone(),
            DotEdgeAttribute::Weight(weight) => format!("weight=\"{}\"", weight),
        }
    }
}

impl IntoDot for Vertex<HashSet<DotVertexAttribute>> {
    fn to_dot(&self) -> String {
        let mut res: String = String::new();
        res.push_str(&format!(
            "\t\"n_{}\" [{}]\n",
            self.id,
            nodes_to_string(&self.data)
        ));
        res
    }
}

impl IntoDot for Edge<HashSet<DotEdgeAttribute>> {
    fn to_dot(&self) -> String {
        let mut res: String = String::new();
        res.push_str(&format!(
            "\t\"n_{}\" -> \"n_{}\" [{}]\n",
            self.from,
            self.to,
            edges_to_string(&self.data)
        ));
        res
    }
}

pub struct DotGraphInfo {
    pub name: String,
    pub color_fn: Option<Box<SetColorCallback>>,
}

impl DotGraphInfo {
    pub fn new(name: &str, color_fn: Option<Box<SetColorCallback>>) -> Self {
        Self {
            name: String::from(name),
            color_fn,
        }
    }
}

pub struct DotGraph {
    pub info: DotGraphInfo,
    digraph: DiGraph<HashSet<DotVertexAttribute>, HashSet<DotEdgeAttribute>>,
}

pub type DotVertex = Vertex<HashSet<DotVertexAttribute>>;
pub type DotEdge = Edge<HashSet<DotEdgeAttribute>>;
pub type SetColorCallback = dyn Fn(&Parameter) -> Color;

impl DotGraph {
    pub fn new(name: &str, color_fn: Option<Box<SetColorCallback>>) -> Self {
        Self {
            info: DotGraphInfo::new(name, color_fn),
            digraph: DiGraph::new(),
        }
    }

    pub fn add_vertex(&mut self, set: &Parameter) {
        let mut vertex: DotVertex = DotVertex::from(set);
        if let Some(f) = &self.info.color_fn {
            vertex.data.insert(DotVertexAttribute::Color(f(set)));
        }
        self.digraph.add_vertex(vertex.id.to_string(), vertex);
    }

    pub fn add_edge(&mut self, edge: DotEdge) {
        self.digraph.add_edge(edge);
    }

    pub fn bfs_get_distance(&self, set: String) -> HashMap<String, usize> {
        self.digraph.bfs_get_distance(set)
    }

    pub fn to_dot(&self) -> String {
        let test_set: HashSet<String> = HashSet::from_iter(self.digraph.vertices.keys().cloned());
        for (from, to) in self.digraph.edges.keys() {
            assert!(test_set.contains(from));
            assert!(test_set.contains(to));
        }
        let mut dot = String::new();
        dot.push_str(&format!("digraph {} {{\n", self.info.name));
        dot.push_str("\tnode [color=lightblue2 style=filled]\n");
        dot.push_str("\tmargin=0.04 size=\"6,6\"\n");
        dot.push_str("\trankdir = BT\n");
        for vertex in self.digraph.vertices.values() {
            dot.push_str(&vertex.to_dot());
        }
        for edge in self.digraph.edges.values() {
            dot.push_str(&edge.to_dot());
        }
        // QoA8jA [label="acyclic chromatic number" URL="./QoA8jA" color="#c5d5e5" shape=box]
        // GNOiyB -> AGnF5Z_dist [label="●" URL="./bound_JngPPm_dist_to_AGnF5Z_dist" arrowhead=onormalnonenonecrow color="#000000" decorate=true fontcolor="#000000" lblstyle="above, sloped" penwidth=2.0 style=solid weight=100]
        dot.push_str("}\n");
        dot
    }

    pub fn save_to_file(&self, target_dir: &Path) -> anyhow::Result<PathBuf> {
        let dot_str = self.to_dot();
        let dot_target_file = target_dir.join(format!("{}.dot", self.info.name));
        file::write_file_content(&dot_target_file, &dot_str)?;
        Ok(dot_target_file)
    }
}

impl From<&Parameter> for DotVertex {
    fn from(set: &Parameter) -> DotVertex {
        let mut data = HashSet::new();
        data.insert(DotVertexAttribute::Label(set.name_core.name.clone()));
        data.insert(DotVertexAttribute::Color(Color::Gray));
        data.insert(DotVertexAttribute::Shape(NodeShape::Box));
        DotVertex {
            id: set.id.to_string(),
            data,
        }
    }
}

fn main() {
    let mut data = HashSet::new();
    data.insert(DotVertexAttribute::Label("carving-width".to_string()));
    data.insert(DotVertexAttribute::Color(Color::Gray));
    data.insert(DotVertexAttribute::Url("./dS6OgO".to_string()));
    data.insert(DotVertexAttribute::Shape(NodeShape::Box));
    let nodes = [DotVertex {
        id: "dS6OgO".to_string(),
        data,
    }];
    fn color_fn(set: &DotVertex) -> String {
        "gray".into()
    };
    // let graph = Graph {
    //     name: "test".into(),
    //     color_fn: Some(Box::new(color_fn)),
    //     nodes,
    //     edges: vec![
    //         Edge {
    //             url: "qq".to_string(),
    //             from: "GNOiyB".to_string(),
    //             to: "AGnF5Z_dist".to_string(),
    //             label: "●".to_string(),
    //             attributes: "URL=\"./bound_JngPPm_dist_to_AGnF5Z_dist\" arrowhead=onormalnonenonecrow color=\"#000000\" decorate=true fontcolor=\"#000000\" lblstyle=\"above, sloped\" penwidth=2.0 style=solid weight=100".to_string(),
    //         },
    //     ],
    // };
    // let dot_string = graph.to_dot();
    // println!("{}", dot_string);
}
