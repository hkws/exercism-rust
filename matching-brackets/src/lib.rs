#[derive(PartialEq)]
enum BracketToken {
    Round,
    Curly,
    Square
}

#[derive(PartialEq)]
enum BracketStatus {
    Open,
    Closed
}

struct Bracket {
    token: BracketToken,
    status: BracketStatus
}

impl Bracket {
    pub fn is_bracket(token: &char) -> bool {
        match token {
            '{' | '[' | '(' | '}' | ']' | ')' => true,
            _ => false
        }
    }
    pub fn new(token: &char) -> Self {
        match token {
            '{' => Self {token: BracketToken::Curly, status: BracketStatus::Open},
            '[' => Self {token: BracketToken::Square, status: BracketStatus::Open},
            '(' => Self {token: BracketToken::Round, status: BracketStatus::Open},
            '}' => Self {token: BracketToken::Curly, status: BracketStatus::Closed},
            ']' => Self {token: BracketToken::Square, status: BracketStatus::Closed},
            ')' => Self {token: BracketToken::Round, status: BracketStatus::Closed},
            _ => unimplemented!()
        }
    }
    pub fn is_paired_with(&self, bracket: &Bracket) -> bool {
        self.token == bracket.token && self.status != bracket.status
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    use std::collections::VecDeque;

    let mut buf = VecDeque::new();
    for ch in string.chars() {
        if !Bracket::is_bracket(&ch) {
            continue
        }
        let brac = Bracket::new(&ch);
        match brac.status {
            BracketStatus::Open => {
                buf.push_back(brac);
            },
            BracketStatus::Closed => {
                if let Some(popped) = buf.pop_back() {
                    if !brac.is_paired_with(&popped) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    };
    buf.is_empty()
}
