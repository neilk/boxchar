use boxchar::game::Game;
use std::io::Write;
use tempfile::NamedTempFile;

// Helper to create temporary test files that clean up automatically
fn create_test_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file
}

#[test]
fn test_valid_game() {
    let content = "ABC\nDEF\nGHI\nJKL\n";
    let file = create_test_file(content);
    let game = Game::from_file(file.path()).unwrap();
    
    assert_eq!(game.sides.len(), 4);
    assert_eq!(game.sides[0], "ABC");
    assert_eq!(game.valid_digraphs.len(), 12 * 9); // 12 letters × 9 possible connections each
}

#[test]
fn test_invalid_number_of_sides() {
    let content = "ABC\nDEF\nGHI\n";  // Only 3 sides
    let file = create_test_file(content);
    let result = Game::from_file(file.path());
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("exactly 4 sides"));
}

#[test]
fn test_uneven_sides() {
    let content = "ABC\nDEF\nGHIJ\nKLM\n";
    let file = create_test_file(content);
    let result = Game::from_file(file.path());
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("same length"));
}

#[test]
fn test_duplicate_letters() {
    let content = "ABC\nDEF\nGHA\nJKL\n";  // 'A' appears twice
    let file = create_test_file(content);
    let result = Game::from_file(file.path());
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Duplicate letter"));
}

#[test]
fn test_lowercase_letters() {
    let content = "ABC\ndef\nGHI\nJKL\n";
    let file = create_test_file(content);
    let result = Game::from_file(file.path());
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("uppercase"));
}

#[test]
fn test_valid_digraphs_generation() {
    let content = "AB\nCD\nEF\nGH\n";
    let file = create_test_file(content);
    let game = Game::from_file(file.path()).unwrap();
    
    // Test specific digraphs that should exist
    assert!(game.valid_digraphs.contains("AC"));
    assert!(game.valid_digraphs.contains("BD"));
    // Test digraphs that shouldn't exist (same side)
    assert!(!game.valid_digraphs.contains("AB"));
    assert!(!game.valid_digraphs.contains("BA"));
}

#[test]
fn test_from_sides_valid_game() {
    let sides = vec!["ABC", "DEF", "GHI", "JKL"]
        .into_iter()
        .map(String::from)
        .collect();
    
    let game = Game::from_sides(sides).unwrap();
    
    assert_eq!(game.sides.len(), 4);
    assert_eq!(game.sides[0], "ABC");
    assert_eq!(game.sides[1], "DEF");
    assert_eq!(game.sides[2], "GHI");
    assert_eq!(game.sides[3], "JKL");
    assert_eq!(game.valid_digraphs.len(), 12 * 9); // 12 letters × 9 possible connections each
}

#[test]
fn test_from_sides_invalid_duplicate_letters() {
    let sides = vec!["ABC", "DEF", "GHA", "JKL"]
        .into_iter()
        .map(String::from)
        .collect();
    
    let result = Game::from_sides(sides);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Duplicate letter"));
}