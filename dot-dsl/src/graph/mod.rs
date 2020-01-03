pub mod graph_items;

use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct Graph {
  pub nodes: Vec<graph_items::node::Node>,
  pub edges: Vec<graph_items::edge::Edge>,
  pub attrs: HashMap<String, String>,
}

impl Graph {
  pub fn new() -> Self {
    Graph {
      attrs: HashMap::new(),
      edges: vec![],
      nodes: vec![],
    }
  }

  pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
    self.nodes = nodes.to_vec();
    self
  }

  pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Self {
    self.edges = edges.to_vec();
    self
  }

  pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
    let to_owned_string = |kvs: &(&str, &str)| (String::from((*kvs).0), String::from((*kvs).1));
    self.attrs = attrs.iter().map(to_owned_string).collect();
    self
  }

  pub fn get_node(&self, name: &str) -> Option<&self::graph_items::node::Node> {
    self.nodes.iter().find(|node| node.name == name)
  }
}

