use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let counter = nucleotide_counts(dna)?;
    println!("{:?}", counter);
    counter.get(&nucleotide).cloned().ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counter = HashMap::new();
    // HashMapにkeyとして AGCTが存在することを保証する。
    // nucleotideでないcharのときのみErrをあげたいので。
    for ch in &['A', 'G', 'C', 'T'] {
        counter.insert(*ch, 0);
    }
    for ch in dna.chars() {
        match ch {
            'A' | 'G' | 'C' | 'T' => {
                let count = counter.entry(ch).or_insert(0);
                *count += 1;
            }
            _ => return Err(ch),
        }
    }
    Ok(counter)
    // https://exercism.io/tracks/rust/exercises/nucleotide-count/solutions/952625ca6a914d2d822ef211c5354c86
    // これでいけちゃうらしい
    // "ACGT".chars().map(|c| (c,count(c,dna))).collect()
    // collectがつよい
}
