pub fn abbreviate(phrase: &str) -> String {
  phrase
    .split_whitespace()
    .map(abbreviate_word)
    .collect::<String>()
}

fn abbreviate_word(word: &str) -> String {
  let alphabetic_words = word
    .trim_matches(|c: char| c.is_ascii_punctuation())
    .split_terminator(|c: char| c.is_ascii_punctuation() && c != '\'');
  alphabetic_words.map(abbreviate_alphabetic_word).collect()
}

fn abbreviate_alphabetic_word(word: &str) -> String {
  let first_char = |word: &str| word.chars().next().map_or(String::new(), |c| c.to_string());

  let mut chars_iter = word.chars();
  let mut abbreviation = String::new();
  chars_iter.next().map(|c| abbreviation.push(c.to_ascii_uppercase()));
  chars_iter
    .filter(char::is_ascii_uppercase)
    .for_each(|c| abbreviation.push(c));
  if abbreviation.len() == word.len() {
    return first_char(&abbreviation);
  }
  abbreviation
}
