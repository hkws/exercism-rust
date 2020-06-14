
pub fn find<T, U>(array: T, key: U) -> Option<usize> 
    where T: AsRef<[U]>, U: Eq+Ord {
    let array = array.as_ref();
    let mut left = 0;
    let mut right = array.len();
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if array[mid] == key {
            return Some(mid);
        }
        if array[mid] < key {
            left = mid;
        } else {
            right = mid;
        }
    }
    if !array.is_empty() && array[left] == key {
        return Some(left);
    }
    None
}

// ここでもやはりgenericsが課題
// やりたいことは、
// ・受け取るarrayがVec, [], Sliceどれでもいいこと
// ・keyは大小比較したり一致確認したりできる型なんでも

// ためしたこと
// ・受け取るarrayがVec, [], Sliceどれでもいいこと -> Derefで[U]になれる
// ・keyは大小比較したり一致確認したりできる -> U: Eq+Ord
// だめでした

// 模範解答
// pub fn find<T, U>(array: T, key: U) -> Option<usize> 
//     where T: AsRef<[U]>, U: Eq+Ord {
//     let array = array.as_ref();
//     ...
// }

// AsRefってなんやねん
// 「"&T"という型になれる型」という指定をしたいときは、hoge: AsRef<T>とすればよい
// その後、hoge.as_ref()で&T型の値が取り出せる。
// つまり、AsRef<T> traitは、&Tへの変換を意味するtraitである

// いつかわかる日が来るだろうメモ
// Borrow と AsRef の使い分け
// Borrow<T> を実装する場合は Self と T でハッシュ値や比較結果が同じであることが求められる。AsRef<T> にはそのような制限は無い。
// HashMap や BTreeMap のキーのようにハッシュ値や比較結果を気にするような場面では Borrow を使い、
// fs::File::open のように様々な型の引数を受け取れる柔軟なインターフェースを実現したい場合には AsRef を使う。

// 型変換についてはここにまとまっているので、定期的に参照したり、コードを書きながら挙動を確認すべし
// https://qiita.com/nirasan/items/e9c621240a7aae914cb8