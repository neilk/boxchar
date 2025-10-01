use boxchar::{board::Board, solver::Solver, dictionary::Dictionary}; // using our library!
use clap::Parser;
use log::debug;
use std::{collections::HashSet, path::Path};

#[derive(Parser)]
#[command(name = "boxchar")]
#[command(about = "A Rust word game application for Letter Boxed puzzles")]
struct Args {
    /// Game specification as comma-separated sides (e.g., "ABC,DEF,GHI,JKL")
    board_spec: Option<String>,

    #[arg(long)]
    board: Option<String>,

    #[arg(long, default_value = "data/dictionary.txt")]
    dictionary: String,

    #[arg(long, default_value_t = 500u16)]
    max_solutions: u16,
}

fn validate_board_spec(board_spec: &str) -> Result<Vec<String>, String> {
    // Check for invalid characters
    for ch in board_spec.chars() {
        if !ch.is_ascii_alphabetic() && ch != ',' {
            return Err(format!("Invalid character '{}' in game specification. Only A-Z, a-z, and commas are allowed.", ch));
        }
    }

    // Split by comma and convert to lowercase
    let sides: Vec<String> = board_spec.split(',').map(|s| s.to_lowercase()).collect();

    if sides.is_empty() {
        return Err("Game specification cannot be empty".to_string());
    }

    Ok(sides)
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = Args::parse();

    let max_solutions = args.max_solutions;

    let dictionary_path = Path::new(&args.dictionary);

    // Handle game - either from positional argument or --game option
    let board = match (&args.board_spec, &args.board) {
        (Some(spec), None) => {
            // Parse comma-separated game specification
            match validate_board_spec(spec) {
                Ok(sides) => {
                    debug!("Loading game from specification: {}", spec);
                    match Board::from_sides(sides) {
                        Ok(game) => game,
                        Err(e) => {
                            eprintln!("Error creating board from specification: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing board specification: {}", e);
                    std::process::exit(1);
                }
            }
        }
        (None, Some(path)) => {
            // Load game from file
            let game_path = Path::new(path);
            debug!("Loading game from: {:?}", game_path);
            match Board::from_path(game_path) {
                Ok(game) => game,
                Err(e) => {
                    eprintln!("Error loading board: {}", e);
                    std::process::exit(1);
                }
            }
        }
        (Some(_), Some(_)) => {
            eprintln!("Error: Cannot specify both board specification and --board option");
            std::process::exit(1);
        }
        (None, None) => {
            eprintln!("Error: Either board specification or --board option is required");
            std::process::exit(1);
        }
    };


    pub fn format_valid_digraphs(digraphs: &HashSet<String>) -> String {
        let mut sorted_digraphs: Vec<_> = digraphs.iter().collect();
        sorted_digraphs.sort();
        sorted_digraphs
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }

    debug!("Successfully loaded game:");
    for (i, side) in board.sides.iter().enumerate() {
        debug!("Side {}: {} ({} letters)", i, side, side.len());
    }
    debug!(
        "Number of valid digraphs in this game: {}",
        board.digraphs.len()
    );
    debug!("Valid digraphs in this game:");
    debug!("{}", format_valid_digraphs(&board.digraphs));

    debug!("Loading dictionary from: {:?}", dictionary_path);
    match Dictionary::from_path(dictionary_path) {
        Ok(dictionary) => {
            solve(board, dictionary, max_solutions);
        }
        Err(e) => eprintln!("Error loading dictionary: {}", e),
    }

    Ok(())
}

fn solve(board: Board, dictionary: Dictionary, max_solutions: u16) {
    debug!("Successfully loaded dictionary:");
    debug!("Number of words: {}", dictionary.words.len());
    {
        let board_dictionary = board.playable_dictionary(&dictionary);
        debug!("\nFirst 10 possible words for this game:");
        for w in board_dictionary.words.iter().take(10) {
            debug!("  {}", w.word);
        }
        debug!("Total possible words: {}", board_dictionary.words.len());

        // Run the solver
        debug!("\nSolving the puzzle...");
        let solver = Solver::new(board, &dictionary, max_solutions);
        let solutions = solver.solve();

        if solutions.is_empty() {
            debug!("No solutions found!");
        } else {
            debug!("Found {} solutions.", solutions.len());
            let mut sorted_solutions = solutions.clone();
            sorted_solutions.sort_by(|a, b| b.score.cmp(&a.score));
            for solution in sorted_solutions.iter() {
                println!("{} {}", solution, solution.score);
            }
        }
    }
}
