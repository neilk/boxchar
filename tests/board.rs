use letter_bounced::board::Board;

#[test]
fn test_from_path() {
    let board = Board::from_path("data/board.txt").unwrap();
    
    assert_eq!(board.sides.len(), 4);
    assert!(!board.digraphs.is_empty());
}