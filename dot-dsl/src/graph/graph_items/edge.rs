use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub struct Edge {
  pub node_a: String,
  pub node_b: String,
  pub attrs: HashMap<String, String>,
}

impl Edge {
  pub fn new(node_a: &str, node_b: &str) -> Self {
    Self {
      node_a: node_a.to_owned(),
      node_b: node_b.to_owned(),
      attrs: HashMap::new(),
    }
  }

  pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
    let to_owned_string = |kvs: &(&str, &str)| (String::from((*kvs).0), String::from((*kvs).1));
    self.attrs = attrs.iter().map(to_owned_string).collect();
    self
  }
}
