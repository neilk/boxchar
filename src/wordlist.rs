use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Wordlist {
    pub words: Vec<String>,
}

impl Wordlist {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let words: Vec<String> = reader
            .lines()
            .filter_map(|line| line.ok())
            .collect();
        
        Ok(Wordlist { words })
    }
}