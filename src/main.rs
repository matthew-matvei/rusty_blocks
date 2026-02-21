use std::{
    cell::RefCell,
    time::{Duration, Instant},
};

use ratatui::{text::Line, widgets::Paragraph, DefaultTerminal};
use rusty_blocks::{
    Block, BlockType, BuildsBlocks, GameBoard, RenderInstruction, RendersGameBoard,
};

fn main() {
    let console_renderer = ConsoleRenderer {
        terminal: RefCell::new(ratatui::init()),
    };
    let mut random_block_builder = RandomBlockBuilder {};
    let mut game_board = GameBoard::new(&console_renderer, &mut random_block_builder);
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_secs(1);
    let mut remaining_ticks = 30;

    game_board.render();

    while remaining_ticks > 0 {
        game_board.render();

        if last_tick.elapsed() >= tick_rate {
            game_board.tick();
            last_tick = Instant::now();
            remaining_ticks = remaining_ticks - 1;
        }
    }
}

struct ConsoleRenderer {
    terminal: RefCell<DefaultTerminal>,
}

impl RendersGameBoard for ConsoleRenderer {
    fn render(&self, instructions: Vec<rusty_blocks::RenderInstruction>) {
        let lines: Vec<Line> = instructions
            .split(|instruction| *instruction == RenderInstruction::NextLine)
            .map(|line| {
                line.iter()
                    .filter_map(|instruction| match instruction {
                        RenderInstruction::Character(character) => Some(character),
                        _ => None,
                    })
                    .collect::<String>()
            })
            .map(|line| Line::from(line))
            .collect();

        self.terminal
            .borrow_mut()
            .draw(|frame| frame.render_widget(Paragraph::new(lines), frame.area()))
            .unwrap();
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
