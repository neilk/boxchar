use boxchar::board::Board;

#[test]
fn test_from_path_data_game() {
    let game = Board::from_path("data/game.txt").unwrap();
    
    assert_eq!(game.sides.len(), 4);
    assert!(!game.valid_digraphs.is_empty());
}