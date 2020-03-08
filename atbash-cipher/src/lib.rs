const ORIGINAL: &'static str = "abcdefghijklmnopqrstuvwxyz1234567890";
const SUBSTITUTION: &'static str = "zyxwvutsrqponmlkjihgfedcba1234567890";

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    replace(plain, ORIGINAL, SUBSTITUTION, |position| {
        if (position + 1) % 6 == 0 { Some(' ') } else { None }
    })
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    replace(cipher, SUBSTITUTION, ORIGINAL, |_| None)
}

fn replace<F>(input: &str, from: &str, to: &str, inject: F) -> String
where
    F: Fn(usize) -> Option<char>,
{
    input.chars().fold(String::from(""), |mut replaced, ch| {
        if let Some(pos) = from
            .chars()
            .position(|subs_ch| subs_ch == ch.to_ascii_lowercase())
        {
            if let Some(inject_char) = inject(replaced.len()) {
                replaced.push(inject_char);
            }
            replaced.push(to.chars().nth(pos).unwrap_or(ch));
        }
        replaced
    })
}
