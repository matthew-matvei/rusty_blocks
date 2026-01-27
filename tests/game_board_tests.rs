use std::cell::RefCell;

use rusty_blocks::*;

#[test]
fn it_renders_an_empty_board_after_initialising() {
    let renderer = TestRenderer::new();
    let mut block_generator = TestBlockBuilder::new();
    GameBoard::new(&renderer, &mut block_generator).render();

    insta::assert_snapshot!(renderer.get_snapshot());
}

#[test]
fn it_moves_a_block_downwards_as_the_game_ticks() {
    let renderer = TestRenderer::new();
    let mut block_generator = TestBlockBuilder::new();
    let mut game_board = GameBoard::new(&renderer, &mut block_generator);

    tick_game_board_times(2, &mut game_board);

    game_board.render();

    insta::assert_snapshot!("board after 2 ticks", renderer.get_snapshot());

    tick_game_board_times(4, &mut game_board);

    game_board.render();

    insta::assert_snapshot!("board after 6 ticks", renderer.get_snapshot());

    tick_game_board_times(8, &mut game_board);

    game_board.render();

    insta::assert_snapshot!("board after 14 ticks", renderer.get_snapshot());
}

#[test]
fn it_loads_a_next_block_once_the_first_reaches_the_bottom() {
    let renderer = TestRenderer::new();
    let mut block_generator = TestBlockBuilder::new();
    let mut game_board = GameBoard::new(&renderer, &mut block_generator);

    tick_game_board_times(21, &mut game_board);

    game_board.render();

    insta::assert_snapshot!("board after 21 ticks", renderer.get_snapshot());

    tick_game_board_times(4, &mut game_board);

    game_board.render();

    insta::assert_snapshot!("board after 25 ticks", renderer.get_snapshot());
}

#[test]
fn it_can_move_a_block_to_the_left_of_the_board() {
    let renderer = TestRenderer::new();
    let mut block_generator = TestBlockBuilder::new();
    let mut game_board = GameBoard::new(&renderer, &mut block_generator);

    tick_game_board_times(4, &mut game_board);

    game_board.move_block(Direction::Left);
    game_board.move_block(Direction::Left);

    game_board.render();

    insta::assert_snapshot!("board after moving left twice", renderer.get_snapshot());

    tick_game_board_times(2, &mut game_board);

    game_board.move_block(Direction::Left);
    game_board.move_block(Direction::Left);
    game_board.move_block(Direction::Left);
    game_board.move_block(Direction::Left);

    game_board.render();

    insta::assert_snapshot!("board after moving too far left", renderer.get_snapshot());
}

#[test]
fn it_can_move_a_block_to_the_right_of_the_board() {
    let renderer = TestRenderer::new();
    let mut block_generator = TestBlockBuilder::new();
    let mut game_board = GameBoard::new(&renderer, &mut block_generator);

    tick_game_board_times(4, &mut game_board);

    game_board.move_block(Direction::Right);
    game_board.move_block(Direction::Right);

    game_board.render();

    insta::assert_snapshot!("board after moving right twice", renderer.get_snapshot());

    tick_game_board_times(2, &mut game_board);

    game_board.move_block(Direction::Right);
    game_board.move_block(Direction::Right);
    game_board.move_block(Direction::Right);
    game_board.move_block(Direction::Right);

    game_board.render();

    insta::assert_snapshot!("board after moving too far right", renderer.get_snapshot());
}

#[test]
fn it_kills_a_block_when_it_ticks_into_a_dead_block() {
    let renderer = TestRenderer::new();
    let mut block_generator = TestBlockBuilder::new();
    let mut game_board = GameBoard::new(&renderer, &mut block_generator);

    tick_game_board_times(25, &mut game_board);

    game_board.render();

    insta::assert_snapshot!("board after 25 ticks", renderer.get_snapshot());

    tick_game_board_times(20, &mut game_board);

    game_board.render();

    insta::assert_snapshot!("board after 45 ticks", renderer.get_snapshot());
}

fn tick_game_board_times<T: RendersGameBoard, V: BuildsBlocks>(
    number_of_times: u8,
    game_board: &mut GameBoard<T, V>,
) {
    for _ in 0..number_of_times {
        game_board.tick();
    }
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

struct TestBlockBuilder {
    sequence_of_block_types: [BlockType; 2],
    position_in_sequence: usize,
}

impl TestBlockBuilder {
    fn new() -> TestBlockBuilder {
        TestBlockBuilder {
            sequence_of_block_types: [BlockType::Square, BlockType::Line],
            position_in_sequence: 0,
        }
    }
}

impl BuildsBlocks for TestBlockBuilder {
    fn build(&mut self) -> Block {
        let next_block_type = self.sequence_of_block_types[self.position_in_sequence];
        if self.position_in_sequence == self.sequence_of_block_types.len() - 1 {
            self.position_in_sequence = 0;
        } else {
            self.position_in_sequence += 1;
        }

        // TODO: Block itself shouldn't need to know where it is in the Game Board
        Block::new(10, next_block_type)
    }
}
