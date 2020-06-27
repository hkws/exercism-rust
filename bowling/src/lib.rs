#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(PartialEq)]
pub enum ThrowingCountType {
    First,
    Second,
    Third
}

pub struct ThrowingCount {
    frame: usize,
    count: ThrowingCountType
}

impl ThrowingCount {
    pub fn new() -> Self {
        Self {
            frame: 1,
            count: ThrowingCountType::First
        }
    }
    pub fn convert_to_frame_index(frame: usize) -> usize {
        frame - 1
    }
    pub fn convert_to_count_index(cnt: &ThrowingCountType) -> usize {
        match cnt {
            ThrowingCountType::First => 0,
            ThrowingCountType::Second => 1,
            ThrowingCountType::Third => 2
        }
    }
    pub fn get_frame_index(&self) -> usize {
        ThrowingCount::convert_to_frame_index(self.frame)
    }
    pub fn get_count_index(&self) -> usize {
        ThrowingCount::convert_to_count_index(&self.count)
    }
}

pub struct BowlingGame {
    score: [[u16; 3]; 10],
    state: ThrowingCount,
    completed: bool
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            score: [[0; 3]; 10],
            state: ThrowingCount::new(),
            completed: false
        }
    }
    pub fn get(&self, frame: usize, throwing_count: ThrowingCountType) -> Option<u16> {
        if !(1..=10).contains(&frame) {
            return None
        }
        let frame_index = ThrowingCount::convert_to_frame_index(frame);
        let count_index = ThrowingCount::convert_to_count_index(&throwing_count);
        Some(self.score[frame_index][count_index])
    }
    pub fn set(&mut self, pins: u16) {
        let frame_index = self.state.get_frame_index();
        let count_index = self.state.get_count_index();
        self.score[frame_index][count_index] = pins;
        self.go_next();
    }
    pub fn frame_sum(&self, frame: usize) -> u16 {
        let frame_idx = ThrowingCount::convert_to_frame_index(frame);
        self.score[frame_idx].iter().sum()
    }
    pub fn go_next(&mut self) {
        // 10フレーム目2投目まででそのrollの和が10ならストライクかスペア -> fill ballを投げる
        if self.state.frame == 10 && self.state.count == ThrowingCountType::Second { 
            if self.frame_sum(self.state.frame) == 10 {
                self.state.count = ThrowingCountType::Third;
            } else {
                self.completed = true;
                return
            }
        };
        match self.state.count {
            ThrowingCountType::First => {
                self.state.count = ThrowingCountType::Second
            }
            ThrowingCountType::Second => {
                self.state.frame += 1;
                self.state.count = ThrowingCountType::First
            }
            ThrowingCountType::Third => {
                self.completed = true;
                return
            }
        };
    }
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.completed {
            return Err(Error::GameComplete)
        }
        let max = 10 - self.get(self.state.frame, ThrowingCountType::First).unwrap();
        if !(0..=max).contains(&pins) {
            return Err(Error::NotEnoughPinsLeft)
        }
        self.set(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.completed {
            return None
        }
        let mut score = 0;
        let mut mul = [1; 2];
        for frame in 1..=10 {
            // scoreを加算
            score += self.frame_sum(frame) * mul[0];
            // mul[0]はもう加算したので次に進める
            mul[0] = mul[1];
            mul[1] = 1;
            // 1投目が10ならstrike -> 2投先まで追加で足す
            if self.get(frame, ThrowingCountType::First).unwrap() == 10 {
                mul[0] += 1;
                mul[1] += 1;
            }
            // 2投目で10になったらspare -> 1投先まで追加で足す
            else if self.frame_sum(frame) == 10 { 
                mul[0] += 1;
            }
        }
        Some(score)
    }
}

// 二次元配列の定義、初期化
// 三項演算子
// NoneをErrに変換
  // ok_or
// enumとusizeの変換