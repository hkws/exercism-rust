pub fn abbreviate(phrase: &str) -> String {
    let mut sphrase: String = phrase.to_string();
    let mut acronym: String = "".to_string();
    sphrase.retain(|c| c.is_alphabetic() || c.is_whitespace() || c == '-');
    for word in sphrase.split_whitespace(){
        acronym += &get_acro(word);
    }
    acronym
}

pub fn get_acro(word: &str) -> String {
    let mut acronym: String = "".to_string();
    if word.contains('-') {
        if word.len() != 1 {
            for s in word.split('-') {
                let ch = s.chars().nth(0).unwrap().to_uppercase().next().unwrap();
                acronym.push(ch);
            }
        }
    } else {
        for (idx, ch) in word.char_indices() {
            if idx == 0 || ch.is_uppercase() {
                acronym.push(ch.to_uppercase().next().unwrap())
            }
        }
    }
    return acronym
}