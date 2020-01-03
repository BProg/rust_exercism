pub fn reply(message: &str) -> &str {
  if message.is_empty() || message.trim().is_empty() {
    return "Fine. Be that way!";
  }

  let is_question = message.trim().ends_with('?');
  let is_yelling = message.chars().any(|c| c.is_alphabetic())
    && message.chars().find(|c| c.is_alphabetic() && c.is_lowercase()).is_none();

  let reply_message: &str;
  match (is_question, is_yelling) {
    (false, false) => reply_message = "Whatever.",
    (false, true) => reply_message = "Whoa, chill out!",
    (true, false) => reply_message = "Sure.",
    (true, true) => reply_message = "Calm down, I know what I'm doing!",
  }
  reply_message
}
