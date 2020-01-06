use std::collections::BTreeMap;

pub fn transform(old_system: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
  let mut new_system: BTreeMap<char, i32> = BTreeMap::new();
  for (points, letters) in old_system {
    for letter in letters {
      new_system.insert(letter.to_ascii_lowercase(), *points);
    }
  }
  new_system
}
