use std::str::Chars;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code_without_spaces = code.replace(" ", "");
    if code_without_spaces.len() <= 1 {
        return false;
    }
    let chs: Chars = code_without_spaces.chars();
    let mut chs_vec: Vec<char> = chs.collect::<Vec<char>>();
    chs_vec.reverse();
    let chs_slice = &chs_vec[..];

    let sum: Option<u32> = chs_slice.chunks(2).fold(Some(0), |sum, chunk| {
        if sum.is_none() {
            return sum;
        }
        let ch_left = chunk.get(0).unwrap_or(&'a');
        let maybe_digit_left = ch_left.to_digit(10);

        let ch_right = chunk.get(1).unwrap_or(&'0');
        let maybe_digit_right = ch_right
            .to_digit(10)
            .and_then(|digit| Some(luhn_double_digit(digit)));
        if let (Some(left), Some(right)) = (maybe_digit_left, maybe_digit_right) {
            Some(sum.unwrap_or(0) + left + right)
        } else {
            None
        }
    });
    sum.map_or_else(|| false, |v| v % 10 == 0)
}

fn luhn_double_digit(digit: u32) -> u32 {
    let double = digit * 2;
    if double > 9 {
        double - 9
    } else {
        double
    }
}
