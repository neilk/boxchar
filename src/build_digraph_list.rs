use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_wordlist() -> io::Result<Vec<String>> {
    let file = File::open("data/wordlist.txt")?;
    all_digraphs(file)
}
    
pub fn all_digraphs(file: File) -> Result<Vec<String>, io::Error> {
    let reader = BufReader::new(file);
    let words: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect();
    
    Ok(words)
}
