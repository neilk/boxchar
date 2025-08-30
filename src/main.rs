use boxchar::{game::Game, wordlist::Wordlist, solver::Solver};  // using our library!
use clap::Parser;
use std::{collections::HashSet, path::Path};

#[derive(Parser)]
#[command(name = "boxchar")]
#[command(about = "A Rust word game application for Letter Boxed puzzles")]
struct Args {
    /// Game specification as comma-separated sides (e.g., "ABC,DEF,GHI,JKL")
    game_spec: Option<String>,
    
    #[arg(long)]
    game: Option<String>,
    
    #[arg(long, default_value = "data/wordlist.txt")]
    wordlist: String,
}

fn validate_game_spec(game_spec: &str) -> Result<Vec<String>, String> {
    // Check for invalid characters
    for ch in game_spec.chars() {
        if !ch.is_ascii_alphabetic() && ch != ',' {
            return Err(format!("Invalid character '{}' in game specification. Only A-Z, a-z, and commas are allowed.", ch));
        }
    }
    
    // Split by comma and convert to uppercase
    let sides: Vec<String> = game_spec
        .split(',')
        .map(|s| s.to_uppercase())
        .collect();
    
    if sides.is_empty() {
        return Err("Game specification cannot be empty".to_string());
    }
    
    Ok(sides)
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    
    let wordlist_path = Path::new(&args.wordlist);
    
    // Handle game - either from positional argument or --game option
    let game = match (&args.game_spec, &args.game) {
        (Some(spec), None) => {
            // Parse comma-separated game specification
            match validate_game_spec(spec) {
                Ok(sides) => {
                    println!("Loading game from specification: {}", spec);
                    match Game::from_sides(sides) {
                        Ok(game) => game,
                        Err(e) => {
                            eprintln!("Error creating game from specification: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing game specification: {}", e);
                    std::process::exit(1);
                }
            }
        }
        (None, Some(path)) => {
            // Load game from file
            let game_path = Path::new(path);
            println!("Loading game from: {:?}", game_path);
            match Game::from_path(game_path) {
                Ok(game) => game,
                Err(e) => {
                    eprintln!("Error loading game: {}", e);
                    std::process::exit(1);
                }
            }
        }
        (Some(_), Some(_)) => {
            eprintln!("Error: Cannot specify both game specification and --game option");
            std::process::exit(1);
        }
        (None, None) => {
            eprintln!("Error: Either game specification or --game option is required");
            std::process::exit(1);
        }
    };
    
    println!("Loading wordlist from: {:?}", wordlist_path);

    pub fn format_valid_digraphs(digraphs: &HashSet<String>) -> String {
        let mut sorted_digraphs: Vec<_> = digraphs.iter().collect();
        sorted_digraphs.sort();
        sorted_digraphs.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ")
    }
    
    println!("Successfully loaded game:");
    for (i, side) in game.sides.iter().enumerate() {
        println!("Side {}: {} ({} letters)", i, side, side.len());
    }
    println!("Number of valid digraphs in this game: {}", game.valid_digraphs.len());
    println!("Valid digraphs in this game:");
    println!("{}", format_valid_digraphs(&game.valid_digraphs));

    match Wordlist::from_path(wordlist_path) {
        Ok(wordlist) => {
            println!("\nSuccessfully loaded wordlist:");
            println!("Number of words: {}", wordlist.words.len());
            println!("First few words: {:?}", &wordlist.words[..5.min(wordlist.words.len())]);
            
            {
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