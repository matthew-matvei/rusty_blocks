use rusty_blocks::*;

#[test]
fn it_renders_an_empty_board_after_initialising() {
    let rendered_board = GameBoard {}.render();

    for row in &rendered_board {
        for element in row {
            assert_eq!(element, "")
        }
    }
}
