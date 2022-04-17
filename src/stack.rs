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
            *v
        } else {
            panic!("stack top is not value: {:?}", instruction)
        }
    }

    pub fn current_instruction(&self) -> Instructions {
        let idx = self.frame_positions.last().unwrap();
        let mut instruction = self.stack.get(*idx).unwrap();
        *instruction
    }

    pub fn current_frame(&self) -> Frame {
        let i = self.current_instruction();
        match i {
            Instructions::Frame(f) => f,
            Instructions::Number(_) => todo!(),
        }
    }

    pub fn next_opcode(&mut self) -> u8 {
        match self
            .current_frame()
            .function
            .expressions
            .get(self.current_frame().get_counter())
        {
            Some(o) => {
                self.current_frame().increment_counter(1);
                println!("c: {}", self.current_frame().get_counter());
                *o
            }
            None => panic!("expression が存在しません。"),
        }
    }

    pub fn current_expression(&self) -> Vec<u8> {
        match self
            .current_frame()
            .function
            .expressions
            .get(self.current_frame().get_counter()..)
        {
            Some(expression) => expression.to_vec(),
            None => panic!("expression が存在しません。"),
        }
    }
}
