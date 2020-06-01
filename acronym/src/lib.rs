pub fn abbreviate(phrase: &str) -> String {
    let mut sphrase: String = phrase.to_string();
    let mut acronym: String = "".to_string();
    sphrase.retain(|c| c.is_alphabetic() || c.is_whitespace() || c == '-');
    for word in sphrase.split_whitespace(){
        acronym += &get_acro(word);
    }
    acronym
}

pub fn get_acro(word: &str) -> String {
    let mut acronym: String = "".to_string();
    if word.contains('-') {
        if word.len() != 1 {
            for s in word.split('-') {
                let ch = s.chars().nth(0).unwrap().to_uppercase().next().unwrap();
                acronym.push(ch);
            }
        }
    } else if word.chars().all(|c| c.is_uppercase()) {
        acronym.push(word.chars().nth(0).unwrap().to_uppercase().next().unwrap());
    } else {
        for (idx, ch) in word.char_indices() {
            if idx == 0 || ch.is_uppercase() {
                acronym.push(ch.to_uppercase().next().unwrap())
            }
        }
    }
    acronym
}

// 悩んだ点
// 1. 文字列の0番目の文字を大文字にして取り出すとき、
// let ch = s.chars().nth(0).unwrap().to_uppercase().next().unwrap();
// としたが、unwrap()いっぱいやってて気持ち悪い
// -> 他の人の回答に
// abbr.push_str(&word.chars().next().unwrap().to_uppercase().collect::<String>());
// ってあったので、これでひとつunwrap()を消せる。でもそれくらい？

// 2. Stringの初期化って"".to_string()でいいのか？
// -> 空文字列なら
// let mut abbr = String::new();
// あと&strから作るなら
// let mut abbr = String::new("hoge");

// 3. 記号を判定する方法ってあるのか？
// 見つからなかったので、今回は事前に
// sphrase.retain(|c| c.is_alphabetic() || c.is_whitespace() || c == '-');
// -> is_記号()みたいな便利なmethodはなさそう。
// s.split(|c: char| !c.is_alphabetic())
// とか、
// name.split(|c: char| c.is_whitespace() || c == '-')
// でなんとかsplitしてた

// 学んだこと
// 1. 文字列の分割をするsplitは、Fnもとれる。なので
// name.split(|c: char| c.is_whitespace() || c == '-')
// だと、空白文字もしくは'-'で分割されることになる。
// -> splitのシグネチャは
// pub fn split<'a, P>(&'a self, pat: P) -> Split<'a, P> where
//     P: Pattern<'a>, 
// なので、引数はcharや&strに限定されない。

// 2. iteratorの一部をiteratorとして取り出す場合は、take()を使う
// ```
// let a = [1, 2, 3];
// let mut iter = a.iter().take(2);
// assert_eq!(iter.next(), Some(&1));
// assert_eq!(iter.next(), Some(&2));
// assert_eq!(iter.next(), None);
// ```

// 3. iteratorのn番目の要素を取り出す場合は、nth()を使う
// 引数は当然0-indexed, またもとのiteratorはconsumeされるので注意
// ```
// let a = [1, 2, 3];
// assert_eq!(a.iter().nth(1), Some(&2));
// ```

// 4. iteratorで特定の条件を満たすまで回したい場合はskip_whileを使う
// ```
// let a = [-1i32, 0, 1];
// let mut iter = a.iter().skip_while(|x| x.is_negative());
// assert_eq!(iter.next(), Some(&0));
// ```