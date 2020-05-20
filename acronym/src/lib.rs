pub fn abbreviate(phrase: &str) -> String {
    let mut sphrase: String = phrase.to_string();
    sphrase.retain(|c| c.is_alphabetic() | c.is_whitespace());
    sphrase.split_whitespace().map::<String>(|word| word.char_indices().filter(|(cnt, ch)| *cnt == 0 as usize || ch.is_uppercase())
                                              .map(|(cnt, ch)| ch.to_uppercase()).collect()).collect()
}
