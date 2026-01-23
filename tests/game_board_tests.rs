use std::cell::RefCell;

use rusty_blocks::*;

#[test]
fn it_renders_an_empty_board_after_initialising() {
    let renderer = TestRenderer::new();
    GameBoard::new(10, 20, &renderer).render();

    insta::assert_snapshot!(renderer.get_snapshot());
}

#[derive(Clone)]
struct TestRenderer {
    snapshot: RefCell<String>,
}

impl TestRenderer {
    fn get_snapshot(&self) -> String {
        self.snapshot.borrow().to_string()
    }

    fn new() -> TestRenderer {
        TestRenderer {
            snapshot: RefCell::new(String::new()),
        }
    }
}

impl RendersGameBoard for TestRenderer {
    fn render(&self, instructions: Vec<rusty_blocks::RenderInstruction>) {
        for instruction in &instructions {
            match instruction {
                RenderInstruction::Character(character) => {
                    self.snapshot.borrow_mut().push(*character)
                }
                RenderInstruction::NextLine => self.snapshot.borrow_mut().push('\n'),
            }
        }
    }
}
