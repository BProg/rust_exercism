pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: Vec<u32> = vec![];
    let mut is_balanced = true;
    let verify_code = |code: u32, brackets: &mut Vec<u32>| {
        if let Some(pop_value) = brackets.pop() {
            return pop_value == code
        } else {
            return false
        };
    };
    for c in string.chars() {
        match c {
            '{' => brackets.push(0),
            '[' => brackets.push(1),
            '(' => brackets.push(2),
            '}' => {
                is_balanced = verify_code(0, &mut brackets);
            }
            ']' => {
                is_balanced = verify_code(1, &mut brackets);
            }
            ')' => {
                is_balanced = verify_code(2, &mut brackets);
            }
            _ => (),
        }
        if !is_balanced {
            break;
        }
    }
    brackets.is_empty() && is_balanced
}
