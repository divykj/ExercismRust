extern crate maplit;
use maplit::hashmap;
use std::collections::HashMap;

macro_rules! impl_attrs {
    () => {
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = (*attrs)
                .into_iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect();
            self
        }
        pub fn get_attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|s| s.as_str())
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Graph {
    pub edges: Vec<Edge>,
    pub nodes: Vec<Node>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: vec![],
            nodes: vec![],
            attrs: hashmap! {},
        }
    }
    pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
        self.nodes = nodes.to_owned();
        self
    }

    pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
        self.edges = edges.to_owned();
        self
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.name == name)
    }

    impl_attrs!();
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub source: String,
    pub destination: String,
    pub attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(source: &str, destination: &str) -> Self {
        Edge {
            source: source.to_string(),
            destination: destination.to_string(),
            attrs: hashmap! {},
        }
    }
    impl_attrs!();
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            attrs: hashmap! {},
        }
    }
    impl_attrs!();
}

pub mod graph {
    pub use super::Graph;
    pub mod graph_items {
        pub mod edge {
            pub use super::super::super::Edge;
        }
        pub mod node {
            pub use super::super::super::Node;
        }
    }
}
