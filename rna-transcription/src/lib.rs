use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct DNA { data: String, }

#[derive(Debug, PartialEq)]
pub struct RNA { data: String, }

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        match validate
        for (i, c) in dna.chars().enumerate() {
            if !"GCTA".contains(c) { return Err(i) }
        }
        Ok( DNA {data: dna.to_string() } )
    }

    fn translate(self) -> String {
        let translation: HashMap<char, char> =
            "GCTA".chars().zip("CGAU".chars()).map(|(a,b)| (a,b)).collect();
        self.data.chars().map(|c| *translation.get(&c).unwrap()).collect()
    }

    pub fn into_rna(self) -> RNA {
        RNA {data: self.translate()}
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, c) in rna.chars().enumerate() {
            if !"CGAU".contains(c) { return Err(i) }
        }
        Ok( RNA {data: rna.to_string() } )
    }
}
