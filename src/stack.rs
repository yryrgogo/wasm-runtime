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

    pub fn push_values(&mut self, num: Number) {
        self.stack.push(Instructions::Number(num));
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

    pub fn current_frame(&self) -> Option<Frame> {
        match self.frame_positions.last() {
            Some(idx) => {
                let instruction = self.stack.get(*idx).unwrap();
                match instruction {
                    Instructions::Frame(f) => Some(f.clone()),
                    Instructions::Number(_) => todo!(),
                }
            }
            None => None,
        }
    }

    pub fn next_opcode(&mut self, frame: &mut Frame) -> u8 {
        let counter = frame.get_counter();
        frame.increment_counter(1);

        let opcode = frame.function.expressions.get(counter).unwrap();
        println!("[next_opcode] opcode: {:x} counter: {}", opcode, counter);
        *opcode
    }

    pub fn current_expression(&self, frame: &mut Frame) -> Vec<u8> {
        match frame.function.expressions.get(frame.get_counter()..) {
            Some(expression) => expression.to_vec(),
            None => panic!("expression が存在しません。"),
        }
    }

    pub fn inspect(&self) -> String {
        format!("Stack size: {}\n{:?}", self.stack.len(), self.stack)
    }
}
