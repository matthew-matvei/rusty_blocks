pub struct GameBoard<'a, T: RendersGameBoard> {
    width: u8,
    height: u8,
    renderer: &'a T,
}

impl<'a, T: RendersGameBoard> GameBoard<'a, T> {
    pub fn render(&self) -> () {
        let mut instructions = Vec::new();
        for _row in 0..self.height {
            instructions.push(RenderInstruction::Character('|'));
            for _col in 0..self.width {
                instructions.push(RenderInstruction::Character(' '));
            }
            instructions.push(RenderInstruction::Character('|'));
            instructions.push(RenderInstruction::NextLine);
        }

        instructions.push(RenderInstruction::Character('|'));

        for _col in 0..self.width {
            instructions.push(RenderInstruction::Character('_'));
        }

        instructions.push(RenderInstruction::Character('|'));

        self.renderer.render(instructions);
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
