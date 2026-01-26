use std::cell::RefCell;

use rusty_blocks::*;

#[test]
fn it_renders_an_empty_board_after_initialising() {
    let renderer = TestRenderer::new();
    GameBoard::new(&renderer).render();

    insta::assert_snapshot!(renderer.get_snapshot());
}

#[test]
fn it_moves_a_block_downwards_as_the_game_ticks() {
    let renderer = TestRenderer::new();
    let mut game_board = GameBoard::new(&renderer);

    game_board.tick();
    game_board.tick();

    game_board.render();

    insta::assert_snapshot!("board after 2 ticks", renderer.get_snapshot());

    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();

    game_board.render();

    insta::assert_snapshot!("board after 6 ticks", renderer.get_snapshot());

    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();

    game_board.render();

    insta::assert_snapshot!("board after 14 ticks", renderer.get_snapshot());
}

#[test]
fn it_loads_a_next_block_once_the_first_reaches_the_bottom() {
    let renderer = TestRenderer::new();
    let mut game_board = GameBoard::new(&renderer);

    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();

    game_board.render();

    insta::assert_snapshot!("board after 22 ticks", renderer.get_snapshot());
}

#[derive(Clone)]
struct TestRenderer {
    snapshot: RefCell<String>,
}

impl TestRenderer {
    fn get_snapshot(&self) -> String {
        self.snapshot.borrow().to_string()
    }

    fn new() -> TestRenderer {
        TestRenderer {
            snapshot: RefCell::new(String::new()),
        }
    }
}

impl RendersGameBoard for TestRenderer {
    fn render(&self, instructions: Vec<rusty_blocks::RenderInstruction>) {
        self.snapshot.borrow_mut().clear();

        for instruction in &instructions {
            match instruction {
                RenderInstruction::Character(character) => {
                    self.snapshot.borrow_mut().push(*character)
                }
                RenderInstruction::NextLine => self.snapshot.borrow_mut().push('\n'),
            }
        }
    }
}
