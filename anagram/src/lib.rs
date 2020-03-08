use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    for possible_anagram in possible_anagrams {
        if are_anagrams(word, possible_anagram) {
            anagrams.insert(possible_anagram);
        }
    }
    anagrams
}

fn are_anagrams(left: &str, right: &str) -> bool {
    let left_lowercase = left.to_lowercase();
    let right_lowercase = right.to_lowercase();
    if left_lowercase == right_lowercase { return false; }
    let to_char_vec = |word: &str| word.chars().collect::<Vec<char>>();
    let mut left_u32 = to_char_vec(&left_lowercase);
    let mut right_u32 = to_char_vec(&right_lowercase);
    left_u32.sort();
    right_u32.sort();
    left_u32 == right_u32
}
