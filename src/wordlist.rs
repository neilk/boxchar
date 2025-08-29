use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Wordlist {
    pub words: Vec<String>,
}

impl Wordlist {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let words: Vec<String> = reader
            .lines()
            .filter_map(|line| line.ok())
            .collect();
        
        Ok(Wordlist { words })
    }
}

// Keep the old functions for backwards compatibility
pub fn read_wordlist() -> io::Result<Vec<String>> {
    let wordlist = Wordlist::from_file("data/wordlist.txt")?;
    Ok(wordlist.words)
}
    
pub fn all_digraphs(file: File) -> Result<Vec<String>, io::Error> {
    let reader = BufReader::new(file);
    let words: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect();
    
    Ok(words)
}
