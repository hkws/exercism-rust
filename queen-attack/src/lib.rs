#[derive(Debug, PartialEq)]
pub struct ChessPosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if ChessPosition::validate(rank, file) {
            return Some(Self { x: rank, y: file });
        }
        None
    }
    pub fn validate(rank: i32, file: i32) -> bool {
        let min = 0;
        let max = 7;
        if (min..max).contains(&rank) && (min..max).contains(&file) {
            return true;
        }
        false
    }
    pub fn duplicate(&self, rank: i32, file: i32) -> bool {
        let pos = ChessPosition::new(rank, file);
        pos.is_some() && self == &pos.unwrap()
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let dx = self.pos.x - other.pos.x;
        let dy = self.pos.y - other.pos.y;
        dx == 0 || dy == 0 || dx.abs() == dy.abs()
    }
}

// 値の範囲のvalidation
// (min..max).contains(&rank)がよさそう

// 数値型の変換
// https://qiita.com/legokichi/items/0f1c592d46a9aaf9a0ea#u64---u32---u16---u8-%E5%AE%89%E5%85%A8%E3%81%AA%E3%83%80%E3%82%A6%E3%83%B3%E3%82%AD%E3%83%A3%E3%82%B9%E3%83%88
// downcastするときはちゃんとvalidationすること

// can attackのfor &whileがキモい？
// 最初に書いたのがこれ
// pub fn can_attack(&self, other: &Queen) -> bool {
//     if self.pos.x == other.pos.x || self.pos.y == other.pos.y {
//         return true;
//     }
//     for (dx, dy) in &[(-1, -1), (-1, 1), (1, -1), (1, 1)] {
//         let (mut nx, mut ny) = (self.pos.x, self.pos.y);
//         while ChessPosition::validate(nx, ny) {
//             if other.pos.duplicate(nx, ny) {
//                 return true;
//             }
//             nx += dx;
//             ny += dy;
//         }
//     }
//     false
// }
// これでいいね、賢い
// pub fn can_attack(&self, other: &Queen) -> bool {
//     let dx = self.pos.x - other.pos.x;
//     let dy = self.pos.y - other.pos.y;
//     dx == 0 || dy == 0 || dx.abs() == dy.abs()
// }
