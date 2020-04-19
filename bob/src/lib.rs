pub fn reply(message: &str) -> &str {
    fn is_question(msg: &str) -> bool {
        msg.ends_with("?")
    }
    fn is_nothing(msg: &str) -> bool {
        if msg.is_empty() {
            return false
        }
        let uppers: Vec<char> = msg.chars().filter(|x| !x.is_whitespace()).collect();
        uppers.len() == msg.len()
    }
    fn is_all_lowercases(msg: &str) -> bool {
        let uppers: Vec<char> = msg.chars().filter(|x| x.is_lowercase()).collect();
        uppers.len() == msg.len()
    }
    match (is_question(message), is_nothing(message), is_all_lowercases(message)) {
        (_, true, _) => "Fine. Be that way!",
        (true, false, false) => "Calm down, I know what I'm doing!",
        (false, false, false) => "Whoa, chill out!",
        (true, false, _) => "Sure.",
        _ => "Whatever."
    }
}
