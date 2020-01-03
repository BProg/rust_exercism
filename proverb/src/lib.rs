pub fn build_proverb(list: &[&str]) -> String {
    let mut final_proverb: Vec<String> = vec![];
    let mut index = 0usize;
    let mut keyword_at_index = list.get(index);
    index += 1;
    let mut keyword_at_next_index = list.get(index);
    while keyword_at_index.is_some() && keyword_at_next_index.is_some() {
        final_proverb.push(build_proverb_part(
            keyword_at_index.unwrap(),
            keyword_at_next_index.unwrap(),
        ));
        keyword_at_index = list.get(index);
        index += 1;
        keyword_at_next_index = list.get(index);
    }
    if let Some(first_keyword) = list.get(0) {
        final_proverb.push(build_root_cause(first_keyword));
    }
    final_proverb.join("\n")
}

fn build_proverb_part(want: &str, lost: &str) -> String {
    format!("For want of a {} the {} was lost.", want, lost)
}

fn build_root_cause(cause: &str) -> String {
    format!("And all for the want of a {}.", cause)
}
