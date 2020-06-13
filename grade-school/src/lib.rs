use std::collections::HashMap;

pub struct School {
    roster: HashMap<u32, Vec<String>>
}

impl School {
    pub fn new() -> School {
        School {roster: HashMap::new()}
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let student_list = self.roster.entry(grade).or_insert(Vec::new()); // returns &mut Vec<String>
        student_list.push(student.to_string());
        student_list.sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.roster.keys().map(|x| x.clone()).collect::<Vec<u32>>(); // returns Vec<u32>
        grades.sort();
        grades
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.roster.get(&grade).cloned()
    }
}

// 所有権まわりのメソッドについて、以下にいい説明があったので記載
// https://qiita.com/lo48576/items/34887794c146042aebf1

// clone(): &self -> self
// 不変参照から所有権を持つオブジェクトを生成。Clone Traitを実装していること。 

// cloned(): Iterator<&T> -> Iterator<T>
// iter.cloned() は、 iter.map(|r| r.clone()) のように動作する。
// (実際の実装では、元のイテレータの next() の結果についてOption::cloned()を呼び出すようになっている。)
// 要するに、参照のイテレータを、所有権のある(cloneされた)オブジェクトのイテレータにするためのものだ。
// なお、参照のイテレータでないものについて cloned() を呼ぶことはできない(型エラーになる)ので、いつでも cloned() を使っておけばいいというものでもない。
// 必要なときにだけ使おう。

// copy(): &self -> self
// 不変参照から所有権を持つオブジェクトを生成することはCloneと変わらない。
// cloneとの大きな違いは、copyは暗黙的に行われるということ。
// すなわち、通常
// fn main() {
//     let s1 = String::from("Hello, Rust");
//     let _s2 = s1;
//     println!("{}", s1); // error
// }
// はエラーになるが、s1が束縛する値がCopy traitを実装していると、暗黙的に値がメモリコピーされ、
// fn main() {
//     let n1 = 5;
//     let _n2 = n1;
//     println!("{}", n1); // 5
// }
// が通る

// copied(): Iterator<&T> -> Iterator<T> (ただし T は Copy)
// 根本的には cloned() と同じだが、 T が単純なメモリコピーで複製できる型に限られたバージョン。
// iter.copied() は、 iter.map(|r| *r) のように (あるいは同じことだが iter.map(|&r| r) のように) 動作する。
// v.clone() だと clone に高いコストがかかる可能性が否めず、それがコードから読み取りづらい。
// 一方、 Copy トレイトが実装された型は、一般に clone (copy) のコストが極めて低い。
// よって、コストが小さなコピーであることが明確になるよう、可能なときは .clone() よりも単純なコピーを、また .cloned() よりも .copied() を使うべきである。

// 所有権まわりが混乱してきたのでまとめておく
// a. 束縛には、2つのパターンがある
//   a-1. 不変な束縛：let x = y;
//   a-2. 可変な束縛：let mut x = y;
//   束縛する変数を変えたいときは、let mutで宣言する
//   もう一つ重要な意味があって、（参照ではなく）所有権を獲得する形で値を束縛したとき、
//   その値を後で書き換えたい（mutableなメソッドを使いたい=第一引数が&mut self）ときは、let mut で宣言する
// b. 参照には、2つのパターンが有る
//   b-1. 不変な参照： &y
//   b-2. 可変な参照： &mut y
//   特定のリソースについて、
//   ・リソースに対する1つ以上の参照（ &T ）
//   ・ただ1つのミュータブルな参照（ &mut T ）
//   のどちらかを持てるが、同時には持てない。
// https://doc.rust-jp.rs/the-rust-programming-language-ja/1.9/book/ownership.html
// https://doc.rust-jp.rs/the-rust-programming-language-ja/1.9/book/references-and-borrowing.html

// というわけでまとめると
// 1. 束縛には可変な束縛と不変な束縛があり、可変な束縛は
// 　・束縛したインスタンスの中身
// 　・束縛するインスタンス自体
// を変更することができることを示す。
// 2. 参照には2パターンあり、&/&mutで取得できる
// 　・可変参照&mutは、変更が可能な参照の取得である。そのため、最初に所有権を取得するときlet mutでなければならない
// 　・可変参照&mutな値をletで束縛するときは、let mut宣言でなくてもよい。
// 　　なぜなら対象のインスタンスが可変でなければ(let mut hoge = String::new()みたいにlet mutで所有権を撮っていなければ)
// 　　そもそも&mutできない、すなわち&mutな時点で可変性が保証されているからである。
// この事により、上記のaddメソッドとgradesメソッドにおいて、
// ・addメソッドではstudent_listは&mutな値を束縛するので、(student_list宣言時においてmutが付いていないが)、sortができる。
// ・gradesメソッドではgrades変数は所有権を獲得するため、このタイミングでmutationを許可するかを宣言する必要があり、let mut gradesとする。 