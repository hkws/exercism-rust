use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let counter = nucleotide_counts(dna).unwrap();
    println!("{:?}", counter);
    counter.get(&nucleotide).cloned().or(Some(0)).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counter = HashMap::new();
    for ch in dna.chars() {
        let count = counter.entry(ch).or_insert(0);
        *count += 1;
    }
    Ok(counter)
}
