
struct RangeWithCustomStep {
    start: u32,
    end: u32,
    step: u32
}

impl Iterator for RangeWithCustomStep {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<u32> {
        if self.start < self.end {
            let v = self.start;
            self.start = match self.step {
                0 => self.end,
                _ => v + self.step
            };
            Some(v)
        } else {
            None
        }
    }
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = factors.iter().map(|x| RangeWithCustomStep{start: *x, end: limit, step: *x})
                                      .map(|x| x.collect::<Vec<u32>>())
                                      .collect::<Vec<Vec<u32>>>()
                                      .concat();
    multiples.sort();
    multiples.dedup();
    multiples.iter().fold(0, |acc, x| acc + x)
    // 模範解答
    // (1..limit).filter(|i| factors.iter().any(|f| i % f == 0)).sum()
    
}
