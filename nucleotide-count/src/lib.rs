use std::collections::HashMap;
const DNA_ACIDS: &str = "GCTA";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match is_valid(nucleotide, dna) {
        Some(err) => Err(err),
        None => { Ok(find_value(nucleotide, dna)) },
    }
}

fn find_value(nucleotide: char, dna: &str) -> usize {
    dna.chars().fold(0, |acc: usize, x| acc + { if x == nucleotide { 1 } else { 0 } })
}

fn find_all_values(dna: &str) -> HashMap<char, usize >{
    HashMap::from(DNA_ACIDS.chars().map(|x| (x, find_value(x, dna))).collect())
}

fn is_valid(nucleotide: char, dna: &str) -> Option<char> {
    if !DNA_ACIDS.contains(nucleotide) {return Some(nucleotide)}
    for c in dna.chars() { if !DNA_ACIDS.contains(c) {return Some(c)} }
    None
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    match is_valid('G', dna) {
        Some(err) => Err(err),
        None => Ok(find_all_values(dna)),
    }
}
