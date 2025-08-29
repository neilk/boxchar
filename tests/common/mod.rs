// Helper to convert string slices to Vec<String> for Game::from_sides
pub fn sides_from_strs(sides: &[&str]) -> Vec<String> {
    sides.iter().map(|s| s.to_string()).collect()
}