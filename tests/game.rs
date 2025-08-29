use boxchar::game::Game;

#[test]
fn test_from_path_data_game() {
    let game = Game::from_path("data/game.txt").unwrap();
    
    assert_eq!(game.sides.len(), 4);
    assert!(!game.valid_digraphs.is_empty());
}