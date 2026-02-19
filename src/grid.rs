pub(crate) struct Grid {
    cells: Vec<bool>,
    width: u8,
}

impl Grid {
    pub(crate) fn get(&self, row: u8, column: u8) -> bool {
        self.cells[(self.width * row + column) as usize]
    }

    pub(crate) fn set(&mut self, row: u8, column: u8) -> () {
        self.cells[(self.width * row + column) as usize] = true;
    }

    pub(crate) fn new(width: u8, height: u8) -> Grid {
        Grid {
            cells: vec![false; (width * height) as usize],
            width,
        }
    }
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

    pub(crate) fn move_down(self) -> Block {
        Block {
            position_in_grid: Point {
                row: self.position_in_grid.row + 1,
                column: self.position_in_grid.column,
            },
            block_type: self.block_type,
        }
    }

    pub(crate) fn move_left(&self) -> Block {
        Block {
            position_in_grid: Point {
                row: self.position_in_grid.row,
                column: self.position_in_grid.column - 1,
            },
            block_type: self.block_type,
        }
    }

    pub(crate) fn move_right(&self) -> Block {
        Block {
            position_in_grid: Point {
                row: self.position_in_grid.row,
                column: self.position_in_grid.column + 1,
            },
            block_type: self.block_type,
        }
    }

    pub(crate) fn covers(self, grid_coordinates: Point) -> bool {
        self.cells().iter().any(|cell| *cell == grid_coordinates)
    }

    pub(crate) fn covers_row(self, row_index: i8) -> bool {
        self.cells().iter().any(|cell| cell.row == row_index)
    }

    pub(crate) fn covers_column(self, column_index: i8) -> bool {
        self.cells().iter().any(|cell| cell.column == column_index)
    }

    pub(crate) fn cells(self) -> Vec<Point> {
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

#[derive(Clone, Copy)]
pub enum BlockType {
    Square,
    Line,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Point {
    pub(crate) row: i8,
    pub(crate) column: i8,
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
