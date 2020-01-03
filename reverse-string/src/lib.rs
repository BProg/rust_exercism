#[cfg(feature = "grapheme")]
use unic_segment::Graphemes;

pub fn reverse(input: &str) -> String {
  #[cfg(not(feature = "grapheme"))]
  return input.chars().rev().collect::<String>();

  #[cfg(feature = "grapheme")]
  return Graphemes::new(input).rev().collect::<String>();
}
