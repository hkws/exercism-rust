pub fn reply(message: &str) -> &str {
    fn is_question(msg: &str) -> bool {
        let _msg: String = msg.chars().filter(|x| !x.is_whitespace()).collect();
        _msg.chars().last() == Some('?')
    }
    fn has_other_characters(msg: &str) -> bool {
        let others: Vec<char> = msg.chars().filter(|x| !x.is_whitespace() && !x.is_lowercase() && !x.is_uppercase()).collect();
        others.len() > 0
    }
    fn has_whitespaces(msg: &str) -> bool {
        if msg.is_empty() {
            return true
        }
        let whites: Vec<char> = msg.chars().filter(|x| x.is_whitespace()).collect();
        whites.len() > 0
    }
    fn has_lowercases(msg: &str) -> bool {
        let lowers: Vec<char> = msg.chars().filter(|x| x.is_lowercase()).collect();
        lowers.len() > 0
    }
    fn has_uppercases(msg: &str) -> bool {
        let uppers: Vec<char> = msg.chars().filter(|x| x.is_uppercase()).collect();
        uppers.len() > 0
    }
    match (is_question(message), has_whitespaces(message), has_lowercases(message), has_uppercases(message), has_other_characters(message)) {
        (true, _, false, true, _) => "Calm down, I know what I'm doing!",
        (false, true, false, false, false) => "Fine. Be that way!",
        (false, _, false, true, _) => "Whoa, chill out!",
        (true, _, _, _, _) => "Sure.",
        _ => "Whatever."
    }
}
