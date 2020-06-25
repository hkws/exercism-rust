// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction 
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            x,
            y,
            direction: d
        }
    }

    pub fn turn_right(mut self) -> Self {
        match self.direction {
            Direction::North => {self.direction = Direction::East},
            Direction::East => {self.direction = Direction::South},
            Direction::South => {self.direction = Direction::West},
            Direction::West => {self.direction = Direction::North},
        }
        self
    }

    pub fn turn_left(mut self) -> Self {
        match self.direction {
            Direction::North => {self.direction = Direction::West},
            Direction::East => {self.direction = Direction::North},
            Direction::South => {self.direction = Direction::East},
            Direction::West => {self.direction = Direction::South},
        }
        self
    }

    pub fn advance(mut self) -> Self {
        match self.direction {
            Direction::North => {self.y += 1},
            Direction::East => {self.x += 1},
            Direction::South => {self.y -= 1},
            Direction::West => {self.x -=1},
        }
        self
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |_self, ch| 
            match ch {
                'R' => {_self.turn_right()},
                'L' => {_self.turn_left()},
                'A' => {_self.advance()},
                _ => unimplemented!()
            }
        )
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

// 困りどころ
// ・Copy traitを実装したくなかった
// -> foldしたおかげで不要になった。forよりIteratorのメソッド使うほうがよさそうやな！
// ・self = self.turn_right();ってキモくない？
//  -> foldしてみたんだけどみんなどうしてるんだろう
// ・チェーンされるメソッド(turn_rightみたいな)をどう書くか
