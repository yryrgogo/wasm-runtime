use crate::instructions::Instructions;
use crate::module::number::Number;
use crate::structure::frame::Frame;

#[derive(Debug)]
pub struct Stack {
    pub stack: Vec<Instructions>,
    pub frame_positions: Vec<usize>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: vec![],
            frame_positions: vec![],
        }
    }

    pub fn push_values(&mut self, values: Vec<Number>) {
        for value in values {
            self.stack.push(Instructions::Number(value));
        }
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.stack.push(Instructions::Frame(frame));
        self.frame_positions.push(self.stack.len() - 1);
    }

    pub fn pop_value(&mut self) -> Number {
        let instruction = self.stack.pop().unwrap();
        match instruction {
            Instructions::Frame(_) => panic!("stack top is not value: {:?}", instruction),
            Instructions::Number(v) => v,
        }
    }

    pub fn peek(&self) -> Number {
        let instruction = self.stack.last().unwrap();
        if let Instructions::Number(v) = instruction {
            v.clone()
        } else {
            panic!("stack top is not value: {:?}", instruction)
        }
    }

    pub fn current_frame(&self) -> Frame {
        let idx = self.frame_positions.last().unwrap();
        let instruction = self.stack.get(*idx).unwrap();
        match instruction {
            Instructions::Frame(ref frame) => frame.clone(),
            Instructions::Number(_) => panic!("position {} is not a Frame", idx),
        }
    }

    pub fn current_expression(&self) -> Vec<u8> {
        let frame = self.current_frame();
        frame.function.expressions
    }
}
