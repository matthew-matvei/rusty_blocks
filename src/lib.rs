pub struct GameBoard<'a, T: RendersGameBoard, V: BuildsBlocks> {
    width: u8,
    height: u8,
    active_block: Option<Block>,
    renderer: &'a T,
    block_builder: &'a V,
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
                .take_if(|block| !block.is_at_row((self.height - 1) as i8))
                .map_or_else(|| self.block_builder.build(), |block| block.move_down()),
        );
    }

    fn instructions_for_row(&self, row_index: u8, instructions: &mut Vec<RenderInstruction>) {
        instructions.push(RenderInstruction::Character('|'));
        for column_index in 0..self.width {
            if self.active_block.map_or(false, |block| {
                block.covers(Point {
                    x: row_index as i8,
                    y: column_index as i8,
                })
            }) {
                instructions.push(RenderInstruction::Character('X'));
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

    pub fn new(renderer: &'a T, block_generator: &'a V) -> GameBoard<'a, T, V> {
        GameBoard {
            width: 10,
            height: 20,
            active_block: None,
            renderer,
            block_builder: block_generator,
        }
    }
}

pub enum RenderInstruction {
    Character(char),
    NextLine,
}

#[derive(Clone, Copy)]
pub struct Block {
    position_in_grid: Point,
}

impl Block {
    pub fn new(grid_width: u8) -> Block {
        Block {
            position_in_grid: Point {
                x: -2,
                y: (grid_width / 2 - 1).try_into().unwrap_or_default(),
            },
        }
    }

    fn move_down(self) -> Block {
        Block {
            position_in_grid: Point {
                x: self.position_in_grid.x + 1,
                y: self.position_in_grid.y,
            },
        }
    }

    fn covers(self, grid_coordinates: Point) -> bool {
        let block_covers_horizontally = grid_coordinates.x == self.position_in_grid.x
            || grid_coordinates.x == self.position_in_grid.x + 1;
        let block_covers_vertically = grid_coordinates.y == self.position_in_grid.y
            || grid_coordinates.y == self.position_in_grid.y + 1;

        block_covers_horizontally && block_covers_vertically
    }

    fn is_at_row(&self, row_index: i8) -> bool {
        row_index == self.position_in_grid.x || row_index == self.position_in_grid.x + 1
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: i8,
    y: i8,
}

pub trait RendersGameBoard {
    fn render(&self, instructions: Vec<RenderInstruction>);
}

pub trait BuildsBlocks {
    fn build(&self) -> Block;
}
