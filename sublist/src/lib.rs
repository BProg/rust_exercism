#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.len() > second_list.len() {
        if contains(first_list, second_list) {
            return Comparison::Superlist;
        }
    }
    if first_list.len() < second_list.len() {
        if contains(second_list, first_list) {
            return Comparison::Sublist;
        }
    }
    if first_list.len() == second_list.len() {
        if contains(first_list, second_list) {
            return Comparison::Equal;
        }
    }
    Comparison::Unequal
}

fn contains<T: PartialEq>(big_list: &[T], small_list: &[T]) -> bool {
    let small_len = small_list.len();
    if small_len == 0 {
        return true;
    }
    big_list
        .windows(small_len)
        .find(|&slice| slice == small_list)
        .is_some()
}
