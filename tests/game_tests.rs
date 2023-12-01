use backwards::GameOfLife;
use std::collections::HashSet;

#[test]
fn test_blinker_pattern() {
    let mut game = crate::GameOfLife::new();
    game.add_cell(-1, 0);
    game.add_cell(0, 0);
    game.add_cell(1, 0);

    game.update();

    let expected_state: HashSet<(i32, i32)> = [(0, -1), (0, 0), (0, 1)].iter().cloned().collect();
    assert_eq!(game.living_cells, expected_state);
}