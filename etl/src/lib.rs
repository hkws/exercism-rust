use std::collections::BTreeMap;


pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut map = BTreeMap::new();
    for (key, value) in h.iter() {
        for ch in value {
            let char_key = ch.to_lowercase().collect::<Vec<char>>();
            map.insert(char_key.first().cloned().unwrap(), *key);
        }
    }
    map
}

// mapは.iter()でpythonでいう.items()ができる
// いいなと思った解は以下 https://exercism.io/tracks/rust/exercises/etl/solutions/b4ac6bcd0ac649a9b398b4ef5a4f8ca3
// pub fn transform(old: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
//     old.iter()
//        .flat_map(|(score, letters)| {
//            letters.iter().map(move |letter| {
//                (letter.to_lowercase(), *score)
//            })
//        })
//        .collect()
// }
// flat_mapは各itemにFnMutを適用した上で、その返り値がiteratorの場合にそれをflatにする