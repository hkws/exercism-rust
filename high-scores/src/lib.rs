#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
    top_three: Vec<Option<u32>>,
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        let mut highscores = Self {
            scores,
            top_three: vec![None, None, None],
        };
        for score in scores {
            highscores.update(*score);
        }
        highscores
    }

    fn update(&mut self, score: u32) {
        let mut tmp: Option<u32> = Some(score);
        for v in self.top_three.iter_mut() {
            if let Some(value) = tmp {
                *v = match *v {
                    Some(n) if n < value => {
                        tmp = Some(n);
                        Some(value)
                    }
                    None => {
                        tmp = None;
                        Some(value)
                    }
                    Some(_) => *v,
                };
            }
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if !self.scores.is_empty() {
            self.scores.last().cloned()
        } else {
            None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if !self.top_three.is_empty() {
            self.top_three[0].clone()
        } else {
            None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three = Vec::with_capacity(3);
        for i in self.top_three.iter() {
            match i {
                Some(n) => top_three.push(*n),
                None => (),
            }
        }
        top_three
    }
}
