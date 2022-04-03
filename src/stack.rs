use crate::instructions::Instructions;
use crate::module::value::Value;
use crate::structure::frame::Frame;

pub struct Stack {
    stack: Vec<Instructions>,
    frame_positions: Vec<usize>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: vec![],
            frame_positions: vec![],
        }
    }

    pub fn push_values(&mut self, mut values: Vec<Instructions>) {
        self.stack.append(&mut values);
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.stack.push(Instructions::Frame(frame));
    }

    pub fn pop_value(&mut self) -> Value {
        let instruction = self.stack.pop().unwrap();
        match instruction {
            Instructions::Frame(_) => panic!("stack top is not value: {:?}", instruction),
            Instructions::Value(v) => v,
        }
    }

    pub fn current_frame(&self) -> &Frame {
        let idx = self.frame_positions.last().unwrap();
        let instruction = self.stack.get(*idx).unwrap();
        match instruction {
            Instructions::Frame(frame) => frame,
            Instructions::Value(_) => panic!("position {} is not a Frame", idx),
        }
    }

    pub fn current_expression(&self) -> Vec<u8> {
        let frame = self.current_frame();
        frame.function.expressions.clone()
    }
}
