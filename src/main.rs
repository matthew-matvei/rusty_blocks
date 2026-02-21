use std::{
    cell::RefCell,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{text::Line, widgets::Paragraph, DefaultTerminal};
use rusty_blocks::{
    Block, BlockType, BuildsBlocks, Direction, GameBoard, RenderInstruction, RendersGameBoard,
};

fn main() {
    let console_renderer = ConsoleRenderer {
        terminal: RefCell::new(ratatui::init()),
    };
    let mut random_block_builder = RandomBlockBuilder {};
    let mut game_board = GameBoard::new(&console_renderer, &mut random_block_builder);
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_secs(1);
    let mut should_exit = false;

    game_board.render();

    while !should_exit {
        game_board.render();
        handle_events(
            tick_rate,
            last_tick,
            || should_exit = true,
            |direction| game_board.move_block(direction),
        );

        if last_tick.elapsed() >= tick_rate {
            game_board.tick();
            last_tick = Instant::now();
        }
    }

    ratatui::restore();
}

fn handle_events(
    tick_rate: Duration,
    last_tick: Instant,
    mut on_quit: impl FnMut() -> (),
    mut on_direction_pressed: impl FnMut(Direction) -> (),
) {
    let timeout = tick_rate.saturating_sub(last_tick.elapsed());

    while event::poll(timeout).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => on_quit(),
                    KeyCode::Left => on_direction_pressed(Direction::Left),
                    KeyCode::Right => on_direction_pressed(Direction::Right),
                    _ => (),
                }
            }
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
