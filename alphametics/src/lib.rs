use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let input_string: String = input.chars().filter(|c| c.is_whitespace()).collect();
    let leftright: Vec<_> = input_string.split("==").collect();
    if leftright.len() != 2 {
        return None
    }
    let lefts = leftright[0].split("+").collect();
    let rights = leftright[1];

}
