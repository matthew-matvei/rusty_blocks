use std::vec;

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

    pub(crate) fn iter(&self) -> GridIterator<'_> {
        self.into_iter()
    }

    pub(crate) fn size(&self) -> usize {
        self.cells.len()
    }

    pub(crate) fn new(width: u8, height: u8) -> Grid {
        Grid {
            cells: vec![false; (width * height) as usize],
            width,
        }
    }

    pub(crate) fn from_cells(cells: Vec<bool>, width: u8, height: u8) -> Grid {
        let new_row_count = height - (cells.len() as u8 / width);
        let mut new_rows = vec![false; (new_row_count * width) as usize];
        new_rows.append(&mut cells.clone());

        Grid {
            cells: new_rows,
            width,
        }
    }
}

pub(crate) struct GridIterator<'a> {
    grid: &'a Grid,
    index: usize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.grid.cells.len() {
            let result =
                Vec::from(&self.grid.cells[self.index..(self.index + (self.grid.width as usize))]);
            self.index += self.grid.width as usize;
            Some(result)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = Vec<bool>;
    type IntoIter = GridIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            grid: self,
            index: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Block {
    position_in_grid: Point,
    block_type: BlockType,
}

impl Block {
    pub fn new(starting_column: u8, block_type: BlockType) -> Block {
        let block_height = match block_type {
            BlockType::Square => 2,
            BlockType::Line => 4,
            BlockType::T => 2,
        };

        Block {
            position_in_grid: Point {
                row: -block_height,
                column: starting_column.try_into().unwrap_or_default(),
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

    pub(crate) fn covers_cells(self, grid: &Grid) -> bool {
        self.cells()
            .iter()
            .filter(|cell| cell.row >= 0)
            .any(|cell| {
                grid.get(
                    cell.row.try_into().unwrap(),
                    cell.column.try_into().unwrap(),
                )
            })
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
            BlockType::T => vec![
                Point { row, column },
                Point {
                    row,
                    column: column + 1,
                },
                Point {
                    row,
                    column: column + 2,
                },
                Point {
                    row: row + 1,
                    column: column + 1,
                },
            ],
        }
    }
}

#[derive(Clone, Copy)]
pub enum BlockType {
    Square,
    Line,
    T,
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

#[test]
fn it_iterates_rows_and_columns_correctly() {
    let mut grid = Grid::new(4, 4);

    grid.set(0, 2);
    grid.set(1, 1);
    grid.set(2, 1);
    grid.set(2, 3);
    grid.set(3, 0);

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, cell) in row.iter().enumerate() {
            assert_eq!(grid.get(row_index as u8, column_index as u8), *cell)
        }
    }
}
