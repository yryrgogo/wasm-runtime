use crate::instructions::Instructions;
use crate::module::function::Block;
use crate::module::number::{
    Number,
    NumberType::{Float32, Float64, Int32, Int64},
};
use crate::structure::frame::Frame;

pub struct Stack {
    pub stack: Vec<Instructions>,
    pub frame_positions: Vec<usize>,
    pub label_positions: Vec<usize>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: vec![],
            frame_positions: vec![],
            label_positions: vec![],
        }
    }

    pub fn push_values(&mut self, mut num: Number) {
        // NOTE: Stack に負の値は push しないため unsigned に変換する
        match num.num_type {
            Int32 => {
                if num.value.i32().is_negative() {
                    let v = 2_u32.pow(31) - num.value.i32().wrapping_abs() as u32 + 2_u32.pow(31);
                    println!("############### eval");
                    num = Number::u32(Some(v));
                }
            }
            Int64 => {
                if num.value.i64() < 0 {
                    let v = num.value.i64() as u64 + 2_u64.pow(64);
                    num = Number::u64(Some(v));
                }
            }
            Float32 => todo!(),
            Float64 => todo!(),
            _ => unreachable!(),
        }

        self.stack.push(Instructions::Number(num));
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.stack.push(Instructions::Frame(frame));
        self.frame_positions.push(self.stack.len() - 1);
    }

    pub fn push_label(&mut self, block: Block) {
        self.stack.push(Instructions::Block(block));
        self.label_positions.push(self.stack.len() - 1);
    }

    pub fn pop_value(&mut self) -> Number {
        let instruction = self.stack.pop().unwrap();
        match instruction {
            Instructions::Number(v) => v,
            _ => panic!("stack top is not value: {:?}", instruction),
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

    pub fn next_opcode(&mut self, frame: &mut Frame) -> u8 {
        let counter = frame.get_counter();
        frame.increment_counter(1);

        let opcode = frame.function.expressions.get(counter).unwrap();
        println!("[next_opcode] opcode: {:x} counter: {}", opcode, counter);
        *opcode
    }

    pub fn current_frame(&self) -> Option<Frame> {
        match self.frame_positions.last() {
            Some(idx) => {
                let instruction = self.stack.get(*idx).unwrap();
                match instruction {
                    Instructions::Frame(f) => Some(f.clone()),
                    _ => unreachable!(),
                }
            }
            None => None,
        }
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
