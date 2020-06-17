pub fn reply(message: &str) -> &str {
    let msg = message.trim();

    let is_empty = msg.is_empty();
    let is_yelling = msg == msg.to_ascii_uppercase() && msg != msg.to_ascii_lowercase();
    let is_question = msg.ends_with("?");

    match (is_empty, is_yelling, is_question) {
        (true, _, _) => "Fine. Be that way!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, true, _) => "Whoa, chill out!",
        (_, _, true) => "Sure.",
        (_, _, _) => "Whatever.",
    }
}
