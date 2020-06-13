use std::ops::Add;

#[derive(PartialEq)]
pub enum TriangleKind {
    Equilateral,
    Isosceles,
    Scalene,
}

pub struct Triangle {
    kind: TriangleKind,
}

impl Triangle {
    pub fn get_triangle_kind<T>(sides: &[T; 3]) -> TriangleKind
    where T: PartialOrd + Copy + Add<T, Output = T> {
        let (first, second, third) = (sides[0], sides[1], sides[2]);
        if first == second && second == third {
            return TriangleKind::Equilateral;
        }
        if first == second || second == third || third == first {
            return TriangleKind::Isosceles;
        }
        TriangleKind::Scalene
    }

    pub fn build<T>(sides: [T; 3]) -> Option<Triangle>
    where T: PartialOrd + Copy + Add<T, Output = T> {
        // validate
        if sides.iter().any(|x| *x == *x + *x) {
            return None;
        }
        for i in 0..=2 {
            let (first, second, third) = (sides[i % 3], sides[(i + 1) % 3], sides[(i + 2) % 3]);
            if first >= second + third {
                return None;
            }
        }

        let kind = Triangle::get_triangle_kind(&sides);
        Some(Self {
            kind
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.kind == TriangleKind::Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self.kind == TriangleKind::Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        self.kind == TriangleKind::Isosceles
    }
}


// 困った点
// PartialEq, Eq, PartialOrd, Ord, 何が違うのか？
// PartialEq: a == b と a != bを保証
// Eq: ↑に加え、 a == aも保証（たとえばf32はstd::f32::NANを含むが、これは同じ者同士の比較でもFalse）
// PartialOrd: 多分比較できる
// Ord: 絶対比較できる（f32はNAN同士の比較ができないのでOrd traitは持たない）

// ジェネリクス
// つよい
// 記述方法として、
// ```
// fn my_fn<T: std::fmt::Debug>(val: T)  {
//     println!("{:?}", val); // Debugを使用できる
// }
// ```
// ```
// fn my_fn<T>(val: T) -> ()
// where
//     T: std::fmt::Debug,
// {
//     println!("{:?}", val); // Debugを使用できる
// }
// ```
// この二種類あるが、whereのほうが表現能力が高いため、基本後者を利用することにする。

// ジェネリクスを使えるのは、ほとんどの型引数を使えるところ
// struct, impl, trait, fn, ...

// 複数のトレイト境界は↓みたいな感じで
// fn my_fn<T, U>(a: T, b: T)
// where
//     T: std::fmt::Display + std::fmt::Debug,
//     U: std::fmt::Display + std::fmt::Debug,
// {
//     println!("{0} {1}", a, b);
//     println!("{0:?} {1:?}", a, b);
// }

// 参考：ジェネリクスの取説 -> https://qiita.com/quasardtm/items/09952838a6ee9582db1d
