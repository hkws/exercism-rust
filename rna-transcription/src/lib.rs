#[derive(Clone, Debug, PartialEq, Eq)]
struct Loc(usize);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Annot<T> {
    value: T,
    loc: Loc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DNANucleotideKind {
    A,
    C,
    G,
    T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RNANucleotideKind {
    A,
    C,
    G,
    U,
}

type DNANucleotide = Annot<DNANucleotideKind>;
impl DNANucleotide {
    fn new(ch: char, loc: usize) -> Result<DNANucleotide, usize> {
        match ch {
            'A' => Ok(Self {
                value: DNANucleotideKind::A,
                loc: Loc(loc),
            }),
            'C' => Ok(Self {
                value: DNANucleotideKind::C,
                loc: Loc(loc),
            }),
            'G' => Ok(Self {
                value: DNANucleotideKind::G,
                loc: Loc(loc),
            }),
            'T' => Ok(Self {
                value: DNANucleotideKind::T,
                loc: Loc(loc),
            }),
            _ => Err(loc),
        }
    }
    fn get_as_char(&self) -> char {
        match self.value {
            DNANucleotideKind::A => 'A',
            DNANucleotideKind::C => 'C',
            DNANucleotideKind::G => 'G',
            DNANucleotideKind::T => 'T',
        }
    }
}

type RNANucleotide = Annot<RNANucleotideKind>;
impl RNANucleotide {
    fn new(ch: char, loc: usize) -> Result<RNANucleotide, usize> {
        match ch {
            'A' => Ok(Self {
                value: RNANucleotideKind::A,
                loc: Loc(loc),
            }),
            'C' => Ok(Self {
                value: RNANucleotideKind::C,
                loc: Loc(loc),
            }),
            'G' => Ok(Self {
                value: RNANucleotideKind::G,
                loc: Loc(loc),
            }),
            'U' => Ok(Self {
                value: RNANucleotideKind::U,
                loc: Loc(loc),
            }),
            _ => Err(loc),
        }
    }
    fn get_as_char(&self) -> char {
        match self.value {
            RNANucleotideKind::A => 'A',
            RNANucleotideKind::C => 'C',
            RNANucleotideKind::G => 'G',
            RNANucleotideKind::U => 'U',
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DNA {
    strands: Vec<DNANucleotide>,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    strands: Vec<RNANucleotide>,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let mut strands = Vec::new();
        for (idx, ch) in dna.char_indices() {
            strands.push(DNANucleotide::new(ch, idx)?);
        }
        Ok(Self { strands })
    }

    pub fn into_rna(self) -> RNA {
        let mut rna_strands = String::new();
        for nuc in self.strands.iter() {
            match nuc.value {
                DNANucleotideKind::A => rna_strands.push('U'),
                DNANucleotideKind::T => rna_strands.push('A'),
                DNANucleotideKind::C => rna_strands.push('G'),
                DNANucleotideKind::G => rna_strands.push('C'),
            }
        }
        RNA::new(&rna_strands).unwrap()
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let mut strands = Vec::new();
        for (idx, ch) in rna.char_indices() {
            strands.push(RNANucleotide::new(ch, idx)?);
        }
        Ok(Self { strands })
    }
}
