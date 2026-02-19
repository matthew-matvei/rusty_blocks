use rusty_blocks::{
    Block, BlockType, BuildsBlocks, GameBoard, RenderInstruction, RendersGameBoard,
};

fn main() {
    let console_renderer = ConsoleRenderer {};
    let mut random_block_builder = RandomBlockBuilder {};
    let mut game_board = GameBoard::new(&console_renderer, &mut random_block_builder);

    game_board.render();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.tick();
    game_board.render();
}

struct ConsoleRenderer;
impl RendersGameBoard for ConsoleRenderer {
    fn render(&self, instructions: Vec<rusty_blocks::RenderInstruction>) {
        let lines_of_instructions =
            instructions.split(|instruction| *instruction == RenderInstruction::NextLine);

        for line in lines_of_instructions {
            let line_to_print: String = line
                .iter()
                .filter_map(|instruction| match instruction {
                    RenderInstruction::Character(character) => Some(character),
                    _ => None,
                })
                .collect();
            println!("{}", line_to_print)
        }
    }
}

struct RandomBlockBuilder;
impl BuildsBlocks for RandomBlockBuilder {
    fn build(&mut self) -> rusty_blocks::Block {
        let random_number: u8 = rand::random_range(0..2);
        match random_number {
            0 => Block::new(10, rusty_blocks::BlockType::Square),
            _ => Block::new(10, BlockType::Line),
        }
    }
}
