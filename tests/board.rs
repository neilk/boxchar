use boxchar::board::Board;

#[test]
fn test_from_path_data_game() {
    let board = Board::from_path("data/game.txt").unwrap();
    
    assert_eq!(board.sides.len(), 4);
    assert!(!board.digraphs.is_empty());
}