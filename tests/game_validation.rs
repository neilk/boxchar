use boxchar::board::Board;

mod common;
use common::sides_from_strs;

#[test]
fn test_valid_game() {
    let sides = sides_from_strs(&["abc", "def", "ghi", "jkl"]);
    let game = Board::from_sides(sides).unwrap();

    assert_eq!(game.sides.len(), 4);
    assert_eq!(game.sides[0], "abc");
    assert_eq!(game.digraphs.len(), 12 * 9); // 12 letters × 9 possible connections each
}

#[test]
fn test_invalid_number_of_sides() {
    let sides = sides_from_strs(&["abc", "def", "ghi"]); // Only 3 sides
    let result = Board::from_sides(sides);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("exactly 4 sides"));
}

#[test]
fn test_uneven_sides() {
    let sides = sides_from_strs(&["abc", "def", "ghij", "klm"]);
    let result = Board::from_sides(sides);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("same length"));
}

#[test]
fn test_duplicate_letters() {
    let sides = sides_from_strs(&["abc", "def", "gha", "jkl"]); // 'A' appears twice
    let result = Board::from_sides(sides);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Duplicate letter"));
}

#[test]
fn test_duplicate_letters_same_side() {
    let sides = sides_from_strs(&["abc", "def", "ghi", "jkj"]); // 'J' appears twice on the same side
    let result = Board::from_sides(sides);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Duplicate letter"));
}

#[test]
fn test_case() {
    let sides = sides_from_strs(&["ABC", "DEF", "ghi", "jkl"]);
    let result = Board::from_sides(sides);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("lowercase"));
}

#[test]
fn test_valid_digraphs_generation() {
    let sides = sides_from_strs(&["ab", "cd", "ef", "gh"]);
    let game = Board::from_sides(sides).unwrap();

    // Test specific digraphs that should exist
    assert!(game.digraphs.contains("ac"));
    assert!(game.digraphs.contains("bd"));
    // Test digraphs that shouldn't exist (same side)
    assert!(!game.digraphs.contains("ab"));
    assert!(!game.digraphs.contains("ba"));
}

#[test]
fn test_from_sides_valid_game() {
    let sides = sides_from_strs(&["abc", "def", "ghi", "jkl"]);

    let game = Board::from_sides(sides).unwrap();

    assert_eq!(game.sides.len(), 4);
    assert_eq!(game.sides[0], "abc");
    assert_eq!(game.sides[1], "def");
    assert_eq!(game.sides[2], "ghi");
    assert_eq!(game.sides[3], "jkl");
    assert_eq!(game.digraphs.len(), 12 * 9); // 12 letters × 9 possible connections each
}

#[test]
fn test_from_sides_invalid_duplicate_letters() {
    let sides = sides_from_strs(&["abc", "def", "gha", "jkl"]);

    let result = Board::from_sides(sides);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Duplicate letter"));
}
