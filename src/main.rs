use boxchar::game::Game;  // using our library!
use std::{collections::HashSet, path::Path};

fn main() -> std::io::Result<()> {
    let game_path = Path::new("data").join("game.txt");
    println!("Loading game from: {:?}", game_path);

    pub fn format_valid_digraphs(digraphs: &HashSet<String>) -> String {
        let mut sorted_digraphs: Vec<_> = digraphs.iter().collect();
        sorted_digraphs.sort();
        sorted_digraphs.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ")
    }
    
    match Game::from_file(game_path) {
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

    Ok(())
}