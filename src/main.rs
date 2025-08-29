use boxchar::game::Game;  // using our library!
use std::path::Path;

fn main() -> std::io::Result<()> {
    let game_path = Path::new("data").join("game.txt");
    println!("Loading game from: {:?}", game_path);
    
    match Game::from_file(game_path) {
        Ok(game) => {
            println!("Successfully loaded game:");
            for (i, side) in game.sides.iter().enumerate() {
                println!("Side {}: {} ({} letters)", i, side, side.len());
            }
            println!("Number of valid digraphs: {}", game.valid_digraphs.len());
            game.print_valid_digraphs();
        }
        Err(e) => println!("Error loading game: {}", e),
    }

    Ok(())
}