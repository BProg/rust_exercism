use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
  pub name: String,
  pub attrs: HashMap<String, String>,
}

impl Node {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_owned(),
      attrs: HashMap::new(),
    }
  }

  pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
    let to_owned_string = |kvs: &(&str, &str)| (String::from((*kvs).0), String::from((*kvs).1));
    self.attrs = attrs.iter().map(to_owned_string).collect();
    self
  }

  pub fn get_attr(&self, key: &str) -> Option<&str> {
    self
      .attrs
      .iter()
      .find_map(|(k, v)| if k == key { Some(v.as_str()) } else { None })
  }
}
