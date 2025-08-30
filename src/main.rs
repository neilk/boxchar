use boxchar::{game::Game, wordlist::Wordlist, solver::Solver};  // using our library!
use clap::Parser;
use std::{collections::HashSet, path::Path};

#[derive(Parser)]
#[command(name = "boxchar")]
#[command(about = "A Rust word game application for Letter Boxed puzzles")]
struct Args {
    #[arg(long)]
    game: Option<String>,
    
    #[arg(long, default_value = "data/wordlist.txt")]
    wordlist: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    
    let wordlist_path = Path::new(&args.wordlist);
    
    // Handle game path - if not provided, print error and exit
    let game_path = match args.game {
        Some(path) => Path::new(&path).to_path_buf(),
        None => {
            eprintln!("Error: --game option is required");
            std::process::exit(1);
        }
    };
    
    println!("Loading game from: {:?}", game_path);
    println!("Loading wordlist from: {:?}", wordlist_path);

    pub fn format_valid_digraphs(digraphs: &HashSet<String>) -> String {
        let mut sorted_digraphs: Vec<_> = digraphs.iter().collect();
        sorted_digraphs.sort();
        sorted_digraphs.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ")
    }
    
    match Game::from_path(&game_path) {
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
            
            if let Ok(game) = Game::from_path(&game_path) {
                let possible_words = game.possible_words(&wordlist);
                println!("\nFirst 10 possible words for this game:");
                for word in possible_words.iter().take(10) {
                    println!("  {}", word);
                }
                println!("Total possible words: {}", possible_words.len());
                
                // Run the solver
                println!("\nSolving the puzzle...");
                let solver = Solver::new(game, wordlist);
                let solutions = solver.solve();
                
                if solutions.is_empty() {
                    println!("No solutions found!");
                } else {
                    println!("Found {} solutions:", solutions.len());
                    for (i, solution) in solutions.iter().take(10).enumerate() {
                        println!("Solution {}: {} words", i + 1, solution.words.len());
                        for word in &solution.words {
                            print!("{} ", word);
                        }
                        println!();
                    }
                    
                    if solutions.len() > 10 {
                        println!("... and {} more solutions", solutions.len() - 10);
                    }
                }
            }
        }
        Err(e) => println!("Error loading wordlist: {}", e),
    }

    Ok(())
}