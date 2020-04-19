# lifetime 
  - メソッドの引数などにより、外部から注入される参照はいつまで有効かわからない。
  - そこで、構造体そのものとlifetimeが同じであることを要求するためにライフタイム指定子を使う
  
# 返り値として所有権を渡すとき
  - Copyしてあげる必要はないか考える
  - 以下はその例で、latestでscores vecの一部を返す際にそのまま返すのではなくCopyして返している

# スライス -> Vec
  - personal_top_threeでやっているように、slice.to_vec()で可能。

# Vec -> スライス
  - &つければOK

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