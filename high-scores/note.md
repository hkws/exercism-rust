# lifetime 
  - メソッドの引数などにより、外部から注入される参照はいつまで有効かわからない。
  - そこで、構造体そのものとlifetimeが同じであることを要求するためにライフタイム指定子を使う
  
# 返り値として所有権を渡すとき
  - Copyしてあげる必要はないか考える
  - 以下はその例で、latestでscores vecの一部を返す際にそのまま返すのではなくCopyして返している

# スライス -> Vec
  - personal_top_threeでやっているように、slice.to_vec()で可能。

# Vec -> スライス
  - &つければOK, derefでスライスになる。厳密には&v[..]

```
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores().iter().last().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores().to_vec();
        scores.sort_unstable();
        scores.into_iter().rev().take(3).collect::<Vec<u32>>()
    }
```

# いろんな変換
## 文字列 -> 数字
let guess: u32 = guess.trim().parse().expect(“Not a number”);
文字列の parse() メソッドは文字列を何かの数値へとパースします。 様々な数値をパース出来るので、Rustに正確にどの型の数値が欲しいのかを伝える必要があります。 なので、 let guess: u32 なのです。

## &str -> String
str.to_string()

## &str -> char
str.chars()

## char -> &str
// https://qiita.com/mHALr/items/26dc38154491d302752b
// 文字列を格納する領域（Rustのstr1文字は最大4バイト）
let mut buffer = [0u8; 4];
let c_ampersand_str: &mut str = c.encode_utf8(&mut buffer);

## charと&str
‘a’はchar
“a”は&str

## &*hogeって何？
&mut hoge -> &hogeにするときは&*hoge？