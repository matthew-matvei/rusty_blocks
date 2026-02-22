pub use crate::grid::Block;
pub use crate::grid::BlockType;
use crate::grid::Grid;
use crate::grid::Point;

mod grid;

pub struct GameBoard<'a, T: RendersGameBoard, V: BuildsBlocks> {
    width: u8,
    height: u8,
    active_block: Option<Block>,
    renderer: &'a T,
    block_builder: &'a mut V,
    dead_cells: Grid,
}

impl<'a, T: RendersGameBoard, V: BuildsBlocks> GameBoard<'a, T, V> {
    pub fn render(&self) -> () {
        let mut instructions = Vec::new();
        for row_index in 0..self.height {
            self.instructions_for_row(row_index, &mut instructions);
        }

        self.instructions_for_bottom(&mut instructions);

        self.renderer.render(instructions);
    }

    pub fn tick(&mut self) -> () {
        self.active_block = Some(
            self.active_block
                .take_if(|block| {
                    let block_is_still_moving = !block.covers_row((self.height - 1) as i8);

                    if !block_is_still_moving || Self::next_row_is_dead(&self.dead_cells, block) {
                        Self::kill(block, &mut self.dead_cells);
                    }

                    block_is_still_moving
                })
                .map_or_else(|| self.block_builder.build(), |block| block.move_down()),
        );
    }

    fn kill(block: &Block, dead_cells: &mut Grid) {
        for cell in block.cells() {
            dead_cells.set(
                cell.row.try_into().unwrap_or_default(),
                cell.column.try_into().unwrap_or_default(),
            );
        }
    }

    // TODO: does 'block' need to be &mut?
    fn next_row_is_dead(dead_cells: &Grid, block: &mut Block) -> bool {
        block.move_down().covers_cells(dead_cells)
    }

    fn instructions_for_row(&self, row_index: u8, instructions: &mut Vec<RenderInstruction>) {
        instructions.push(RenderInstruction::Character('|'));
        for column_index in 0..self.width {
            if self.dead_cells.get(row_index, column_index) {
                instructions.push(RenderInstruction::Character('x'));
            } else if self.active_block.map_or(false, |block| {
                block.covers(Point {
                    row: row_index as i8,
                    column: column_index as i8,
                })
            }) {
                instructions.push(RenderInstruction::Character('+'));
            } else {
                instructions.push(RenderInstruction::Character(' '));
            }
        }
        instructions.push(RenderInstruction::Character('|'));
        instructions.push(RenderInstruction::NextLine);
    }

    fn instructions_for_bottom(&self, instructions: &mut Vec<RenderInstruction>) {
        for _col in 0..self.width + 2 {
            instructions.push(RenderInstruction::Character('-'));
        }
    }

    pub fn new(renderer: &'a T, block_builder: &'a mut V) -> GameBoard<'a, T, V> {
        GameBoard {
            width: 10,
            height: 20,
            active_block: None,
            renderer,
            block_builder,
            dead_cells: Grid::new(10, 20),
        }
    }

    pub fn move_block(&mut self, direction: Direction) -> () {
        self.active_block = self.active_block.map(|block| {
            match direction {
                Direction::Left => {
                    if block.covers_column(0) {
                        None
                    } else {
                        Some(block.move_left())
                    }
                }
                Direction::Right => {
                    if block.covers_column((self.width - 1) as i8) {
                        None
                    } else {
                        Some(block.move_right())
                    }
                }
                Direction::Down => {
                    if block.covers_row((self.height - 1) as i8) {
                        None
                    } else {
                        Some(block.move_down())
                    }
                }
            }
            .take_if(|block| !block.covers_cells(&self.dead_cells))
            .unwrap_or(block)
        })
    }
}

#[derive(PartialEq, Eq)]
pub enum RenderInstruction {
    Character(char),
    NextLine,
}

pub trait RendersGameBoard {
    fn render(&self, instructions: Vec<RenderInstruction>);
}

pub trait BuildsBlocks {
    fn build(&mut self) -> Block;
}

pub enum Direction {
    Left,
    Right,
    Down,
}
