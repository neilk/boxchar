use boxchar::{game::Game, wordlist::Wordlist};  // using our library!
use std::{collections::HashSet, path::Path};

fn main() -> std::io::Result<()> {
    let game_path = Path::new("data").join("game.txt");
    let wordlist_path = Path::new("data").join("wordlist.txt");
    
    println!("Loading game from: {:?}", game_path);
    println!("Loading wordlist from: {:?}", wordlist_path);

    pub fn format_valid_digraphs(digraphs: &HashSet<String>) -> String {
        let mut sorted_digraphs: Vec<_> = digraphs.iter().collect();
        sorted_digraphs.sort();
        sorted_digraphs.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ")
    }
    
    match Game::from_path(game_path) {
        Ok(game) => {
            println!("Successfully loaded game:");
            for (i, side) in game.sides.iter().enumerate() {
                println!("Side {}: {} ({} letters)", i, side, side.len());
            }
            println!("Number of valid digraphs in this game: {}", game.valid_digraphs.len());
            println!("Valid digraphs in this game:");
            println!("{}", format_valid_digraphs(&game.valid_digraphs));
        }
        Err(e) => println!("Error loading game: {}", e),
    }

    match Wordlist::from_path(wordlist_path) {
        Ok(wordlist) => {
            println!("\nSuccessfully loaded wordlist:");
            println!("Number of words: {}", wordlist.words.len());
            println!("First few words: {:?}", &wordlist.words[..5.min(wordlist.words.len())]);
            
            if let Ok(game) = Game::from_path("data/game.txt") {
                let possible_words = game.possible_words(&wordlist);
                println!("\nFirst 10 possible words for this game:");
                for word in possible_words.iter().take(10) {
                    println!("  {}", word);
                }
                println!("Total possible words: {}", possible_words.len());
            }
        }
        Err(e) => println!("Error loading wordlist: {}", e),
    }

    Ok(())
}