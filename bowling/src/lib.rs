#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq)]
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
    pub fn get(&self, frame: usize, throwing_count: &ThrowingCountType) -> Option<u16> {
        if !(1..=10).contains(&frame) {
            return None
        }
        let frame_index = ThrowingCount::convert_to_frame_index(frame);
        let count_index = ThrowingCount::convert_to_count_index(throwing_count);
        Some(self.score[frame_index][count_index])
    }
    pub fn set(&mut self, pins: u16) {
        let frame_index = self.state.get_frame_index();
        let count_index = self.state.get_count_index();
        self.score[frame_index][count_index] = pins;
        let striked = self.state.frame != 10 && self.state.count == ThrowingCountType::First && pins == 10;
        self.go_next(striked);
    }
    pub fn frame_sum(&self, frame: usize) -> u16 {
        let frame_idx = ThrowingCount::convert_to_frame_index(frame);
        self.score[frame_idx].iter().sum()
    }
    pub fn go_next(&mut self, striked: bool) {
        // 10フレーム目2投目まででそのrollの和が10ならストライクかスペア -> fill ballを投げる
        if self.state.frame == 10 && self.state.count == ThrowingCountType::Second { 
            if self.frame_sum(self.state.frame) >= 10 {
                self.state.count = ThrowingCountType::Third;
            } else {
                self.completed = true;
            }
            return
        };
        match self.state.count {
            ThrowingCountType::First => {
                if striked { 
                    self.state.frame += 1;
                    self.state.count = ThrowingCountType::First;
                } else {
                    self.state.count = ThrowingCountType::Second;
                }
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
        let max = if self.state.frame != 10 {
            10 - self.get(self.state.frame, &ThrowingCountType::First).unwrap()
        } else {
            match self.state.count {
                ThrowingCountType::First => 10,
                ThrowingCountType::Second => 10,
                ThrowingCountType::Third => if 10 == self.get(self.state.frame, &ThrowingCountType::Second).unwrap() || 10 == self.get(self.state.frame, &ThrowingCountType::First).unwrap() + self.get(self.state.frame, &ThrowingCountType::Second).unwrap() {
                    10
                } else {
                    10 - self.get(self.state.frame, &ThrowingCountType::Second).unwrap()
                },
            }
        };
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
        let mut frame = 1;
        while frame <= 10 {
            for count in [ThrowingCountType::First,
                          ThrowingCountType::Second,
                          ThrowingCountType::Third].iter() {
                if frame != 10 && count == &ThrowingCountType::Third {
                    continue
                }
                // scoreを加算
                let pins = self.get(frame, count).unwrap();
                score +=  pins * mul[0];

                // mul[0]はもう加算したので次に進める
                mul[0] = mul[1];
                mul[1] = 1;
                if frame != 10 {
                    // 1投目が10ならstrike -> 2投先まで追加で足す
                    if count == &ThrowingCountType::First && pins == 10 {
                        mul[0] += 1;
                        mul[1] += 1;
                        break
                    }
                    // 2投目で10になったらspare -> 1投先まで追加で足す
                    else if count == &ThrowingCountType::Second && self.frame_sum(frame) == 10 { 
                        mul[0] += 1;
                    }
                }
            }
            frame += 1;
        }
        Some(score)
    }
}

// 二次元配列の定義、初期化
// こんなイメージ
// pub struct BowlingGame {
//     score: [[u16; 3]; 10],
//     state: ThrowingCount,
//     completed: bool
// }

// impl BowlingGame {
//     pub fn new() -> Self {
//         Self {
//             score: [[0; 3]; 10],
//             state: ThrowingCount::new(),
//             completed: false
//         }
//     }
// }

// 三項演算子
// こんな感じで、ifは式として書ける
// match self.state.count {
//     ThrowingCountType::First => 10,
//     ThrowingCountType::Second => 10,
//     ThrowingCountType::Third => if 10 == self.get(self.state.frame, &ThrowingCountType::Second).unwrap() || 10 == self.get(self.state.frame, &ThrowingCountType::First).unwrap() + self.get(self.state.frame, &ThrowingCountType::Second).unwrap() {
//         10
//     } else {
//         10 - self.get(self.state.frame, &ThrowingCountType::Second).unwrap()
//     },
// }

// NoneをErrに変換
  // ok_or

// enumとusizeの変換
// かゆいところにも手が届く神言語
// https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum/c_like.html
// An attribute to hide warnings for unused code.
// #![allow(dead_code)]

// // enum with implicit discriminator (starts at 0)
// enum Number {
//     Zero,
//     One,
//     Two,
// }

// // enum with explicit discriminator
// enum Color {
//     Red = 0xff0000,
//     Green = 0x00ff00,
//     Blue = 0x0000ff,
// }

// fn main() {
//     // `enums` can be cast as integers.
//     println!("zero is {}", Number::Zero as i32);
//     println!("one is {}", Number::One as i32);

//     println!("roses are #{:06x}", Color::Red as i32);
//     println!("violets are #{:06x}", Color::Blue as i32);
// }

// 全体的にコードが汚い
// どのコードもサクッときれいに書けているわけではないけど、↓はかなり見通しがよかった。
// frameの持ち方とscoreの算出方法がよい
// https://exercism.io/tracks/rust/exercises/bowling/solutions/43d892f26373478db9ea5fc72f82595a