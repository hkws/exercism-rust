// むずすぎるので保留
// そのうち再チャレンジする

use std::slice::Permutations;
use std::collections::HashMap;
use itertools::Itertools;

struct MapFactory {

}



struct Solver {
    characters: Vec<char>,
    lefts: Vec<String>,
    right: String
}

impl Solver {
    pub fn new(lefts: Vec<String>, right: String) -> Self {
        let sets = Vec::new();
        for token in lefts.iter().chain([right].into_iter()) {
            for ch in token.chars() {
                sets.push(ch);
            }
        }
        Self {
            characters: sets,
            lefts, 
            right
        }
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let input_string: String = input.chars().filter(|c| c.is_whitespace()).collect();
    let leftright: Vec<_> = input_string.split("==").collect();
    if leftright.len() != 2 {
        return None
    }
    let lefts = leftright[0].split("+").map(|token| token.to_string()).collect();
    let right = leftright[1].to_string();
    let solver = Solver::new(lefts, right);
}
