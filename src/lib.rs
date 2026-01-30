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
                    let block_is_still_moving = !block.reached_row((self.height - 1) as i8);

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

    fn next_row_is_dead(dead_cells: &Grid, block: &mut Block) -> bool {
        block
            .move_down()
            .cells()
            .iter()
            .filter(|cell| cell.row >= 0)
            .any(|cell| {
                dead_cells.get(
                    cell.row.try_into().unwrap(),
                    cell.column.try_into().unwrap(),
                )
            })
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

    pub fn new(renderer: &'a T, block_generator: &'a mut V) -> GameBoard<'a, T, V> {
        GameBoard {
            width: 10,
            height: 20,
            active_block: None,
            renderer,
            block_builder: block_generator,
            dead_cells: Grid::new(10, 20),
        }
    }

    pub fn move_block(&mut self, direction: Direction) -> () {
        self.active_block = self.active_block.map(|block| match direction {
            Direction::Left => Block {
                position_in_grid: Point {
                    row: block.position_in_grid.row,
                    column: if block.cells().iter().any(|cell| cell.column == 0) {
                        block.position_in_grid.column
                    } else {
                        block.position_in_grid.column - 1
                    },
                },
                block_type: block.block_type,
            },
            Direction::Right => Block {
                position_in_grid: Point {
                    row: block.position_in_grid.row,
                    column: if block
                        .cells()
                        .iter()
                        .any(|cell| cell.column == (self.width - 1) as i8)
                    {
                        block.position_in_grid.column
                    } else {
                        block.position_in_grid.column + 1
                    },
                },
                block_type: block.block_type,
            },
        })
    }
}

pub enum RenderInstruction {
    Character(char),
    NextLine,
}

#[derive(Clone, Copy)]
pub struct Block {
    position_in_grid: Point,
    block_type: BlockType,
}

impl Block {
    pub fn new(grid_width: u8, block_type: BlockType) -> Block {
        Block {
            position_in_grid: Point {
                row: -2,
                column: (grid_width / 2 - 1).try_into().unwrap_or_default(),
            },
            block_type,
        }
    }

    fn move_down(self) -> Block {
        Block {
            position_in_grid: Point {
                row: self.position_in_grid.row + 1,
                column: self.position_in_grid.column,
            },
            block_type: self.block_type,
        }
    }

    fn covers(self, grid_coordinates: Point) -> bool {
        self.cells().iter().any(|cell| *cell == grid_coordinates)
    }

    fn reached_row(self, row_index: i8) -> bool {
        self.cells().iter().any(|cell| cell.row == row_index)
    }

    fn cells(self) -> Vec<Point> {
        let row = self.position_in_grid.row;
        let column = self.position_in_grid.column;

        match self.block_type {
            BlockType::Square => vec![
                Point { row, column },
                Point {
                    row: row + 1,
                    column,
                },
                Point {
                    row,
                    column: column + 1,
                },
                Point {
                    row: row + 1,
                    column: column + 1,
                },
            ],
            BlockType::Line => vec![
                Point { row, column },
                Point {
                    row: row + 1,
                    column,
                },
                Point {
                    row: row + 2,
                    column,
                },
                Point {
                    row: row + 3,
                    column,
                },
            ],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    row: i8,
    column: i8,
}

pub trait RendersGameBoard {
    fn render(&self, instructions: Vec<RenderInstruction>);
}

pub trait BuildsBlocks {
    fn build(&mut self) -> Block;
}

#[derive(Clone, Copy)]
pub enum BlockType {
    Square,
    Line,
}

struct Grid {
    cells: Vec<bool>,
    width: u8,
}

impl Grid {
    fn get(&self, row: u8, column: u8) -> bool {
        self.cells[(self.width * row + column) as usize]
    }

    fn set(&mut self, row: u8, column: u8) -> () {
        self.cells[(self.width * row + column) as usize] = true;
    }

    fn new(width: u8, height: u8) -> Grid {
        Grid {
            cells: vec![false; (width * height) as usize],
            width,
        }
    }
}

#[test]
fn it_gets_and_sets_correct_grid_cells() {
    let mut grid = Grid::new(10, 20);

    assert!(!grid.get(0, 0));
    grid.set(0, 0);
    assert!(grid.get(0, 0));

    assert!(!grid.get(5, 2));
    grid.set(5, 2);
    assert!(grid.get(5, 2));
    assert!(!grid.get(2, 5));

    assert!(!grid.get(9, 19));
    grid.set(9, 19);
    assert!(grid.get(9, 19));
    assert!(!grid.get(19, 9));

    for row in 3..5 {
        for column in 2..4 {
            grid.set(row, column);
        }
    }

    assert!(grid.get(3, 2));
    assert!(!grid.get(5, 4));
}

pub enum Direction {
    Left,
    Right,
}
