const ORIGINAL: &'static str = "abcdefghijklmnopqrstuvwxyz1234567890";
const SUBSTITUTION: &'static str = "zyxwvutsrqponmlkjihgfedcba1234567890";

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.chars().fold(String::from(""), |mut encoded, ch| {
        if let Some(pos) = ORIGINAL
            .chars()
            .position(|original_ch| original_ch == ch.to_ascii_lowercase())
        {
            if (encoded.len() + 1) % 6 == 0 {
                encoded.push(' ');
            }
            encoded.push(SUBSTITUTION.chars().nth(pos).unwrap_or(ch));
        }
        encoded
    })
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().fold(String::from(""), |mut decoded, ch| {
        if let Some(pos) = SUBSTITUTION
            .chars()
            .position(|subs_ch| subs_ch == ch.to_ascii_lowercase())
        {
            decoded.push(ORIGINAL.chars().nth(pos).unwrap_or(ch));
        }
        decoded
    })
}
