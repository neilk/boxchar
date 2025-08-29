use boxchar::game::Game;

// Helper to convert string slices to Vec<String> for Game::from_sides
fn sides_from_strs(sides: &[&str]) -> Vec<String> {
    sides.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_valid_game() {
    let sides = sides_from_strs(&["ABC", "DEF", "GHI", "JKL"]);
    let game = Game::from_sides(sides).unwrap();
    
    assert_eq!(game.sides.len(), 4);
    assert_eq!(game.sides[0], "ABC");
    assert_eq!(game.valid_digraphs.len(), 12 * 9); // 12 letters × 9 possible connections each
}

#[test]
fn test_invalid_number_of_sides() {
    let sides = sides_from_strs(&["ABC", "DEF", "GHI"]); // Only 3 sides
    let result = Game::from_sides(sides);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("exactly 4 sides"));
}

#[test]
fn test_uneven_sides() {
    let sides = sides_from_strs(&["ABC", "DEF", "GHIJ", "KLM"]);
    let result = Game::from_sides(sides);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("same length"));
}

#[test]
fn test_duplicate_letters() {
    let sides = sides_from_strs(&["ABC", "DEF", "GHA", "JKL"]); // 'A' appears twice
    let result = Game::from_sides(sides);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Duplicate letter"));
}

#[test]
fn test_lowercase_letters() {
    let sides = sides_from_strs(&["ABC", "def", "GHI", "JKL"]);
    let result = Game::from_sides(sides);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("uppercase"));
}

#[test]
fn test_valid_digraphs_generation() {
    let sides = sides_from_strs(&["AB", "CD", "EF", "GH"]);
    let game = Game::from_sides(sides).unwrap();
    
    // Test specific digraphs that should exist
    assert!(game.valid_digraphs.contains("AC"));
    assert!(game.valid_digraphs.contains("BD"));
    // Test digraphs that shouldn't exist (same side)
    assert!(!game.valid_digraphs.contains("AB"));
    assert!(!game.valid_digraphs.contains("BA"));
}

#[test]
fn test_from_sides_valid_game() {
    let sides = sides_from_strs(&["ABC", "DEF", "GHI", "JKL"]);
    
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
    let sides = sides_from_strs(&["ABC", "DEF", "GHA", "JKL"]);
    
    let result = Game::from_sides(sides);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Duplicate letter"));
}