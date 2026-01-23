pub struct GameBoard<'a, T: RendersGameBoard> {
    width: u8,
    height: u8,
    renderer: &'a T,
}

impl<'a, T: RendersGameBoard> GameBoard<'a, T> {
    pub fn render(&self) -> () {
        let mut instructions = Vec::new();
        for _row in 0..self.height {
            self.instructions_for_row(&mut instructions);
        }

        self.instructions_for_bottom(&mut instructions);

        self.renderer.render(instructions);
    }

    fn instructions_for_row(&self, instructions: &mut Vec<RenderInstruction>) {
        instructions.push(RenderInstruction::Character('|'));
        for _col in 0..self.width {
            instructions.push(RenderInstruction::Character(' '));
        }
        instructions.push(RenderInstruction::Character('|'));
        instructions.push(RenderInstruction::NextLine);
    }

    fn instructions_for_bottom(&self, instructions: &mut Vec<RenderInstruction>) {
        instructions.push(RenderInstruction::Character('|'));

        for _col in 0..self.width {
            instructions.push(RenderInstruction::Character('_'));
        }

        instructions.push(RenderInstruction::Character('|'));
    }

    pub fn new(width: u8, height: u8, renderer: &'a T) -> GameBoard<'a, T> {
        GameBoard {
            width,
            height,
            renderer,
        }
    }
}

pub enum RenderInstruction {
    Character(char),
    NextLine,
}

pub trait RendersGameBoard {
    fn render(&self, instructions: Vec<RenderInstruction>);
}
