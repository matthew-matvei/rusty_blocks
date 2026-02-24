use std::{
    cell::RefCell,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{text::Line, widgets::Paragraph, DefaultTerminal};
use rusty_blocks::{
    Block, BlockType, BuildsBlocks, Direction, GameBoard, RenderInstruction, RendersGameBoard,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let console_renderer = ConsoleRenderer {
        terminal: RefCell::new(ratatui::init()),
    };
    let mut random_block_builder = RandomBlockBuilder {};
    let mut game_board = GameBoard::new(&console_renderer, &mut random_block_builder);
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_secs(1);
    let mut should_exit = false;

    while !should_exit {
        game_board.render();

        handle_keyboard_events(|| should_exit = true, &mut game_board)?;

        if last_tick.elapsed() >= tick_rate {
            game_board.tick();
            last_tick = Instant::now();
        }
    }

    ratatui::restore();
    disable_raw_mode()?;

    Ok(())
}

fn handle_keyboard_events(
    mut on_quit: impl FnMut() -> (),
    game_board: &mut GameBoard<'_, ConsoleRenderer, RandomBlockBuilder>,
) -> io::Result<()> {
    let timeout = Duration::from_millis(10);

    while event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => on_quit(),
                    KeyCode::Left => game_board.move_block(Direction::Left),
                    KeyCode::Right => game_board.move_block(Direction::Right),
                    KeyCode::Down => game_board.move_block(Direction::Down),
                    KeyCode::Char(' ') => game_board.rotate_block(),
                    _ => (),
                }
            }
        }
    }

    Ok(())
}

struct ConsoleRenderer {
    terminal: RefCell<DefaultTerminal>,
}

impl RendersGameBoard for ConsoleRenderer {
    fn render(&self, instructions: Vec<rusty_blocks::RenderInstruction>) {
        let lines: Vec<Line> = instructions
            .split(by_line)
            .map(instructions_to_string)
            .map(Line::from)
            .collect();

        self.terminal
            .borrow_mut()
            .draw(|frame| frame.render_widget(Paragraph::new(lines), frame.area()))
            .unwrap();
    }
}

fn by_line(instruction: &RenderInstruction) -> bool {
    *instruction == RenderInstruction::NextLine
}

fn instructions_to_string(line_of_instructions: &[RenderInstruction]) -> String {
    line_of_instructions
        .iter()
        .filter_map(|instruction| match instruction {
            RenderInstruction::Character(character) => Some(character),
            _ => None,
        })
        .collect::<String>()
}

struct RandomBlockBuilder;
impl BuildsBlocks for RandomBlockBuilder {
    fn build(&mut self) -> rusty_blocks::Block {
        let random_number: u8 = rand::random_range(0..3);
        match random_number {
            0 => Block::new(4, rusty_blocks::BlockType::Square),
            1 => Block::new(4, BlockType::T),
            _ => Block::new(4, BlockType::Line),
        }
    }
}
