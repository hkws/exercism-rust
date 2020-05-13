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
