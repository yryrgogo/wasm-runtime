use crate::instructions::Instructions;
use crate::module::function::Block;
use crate::module::number::{
    Number,
    NumberType::{Float32, Float64, Int32, Int64, Uint32, Uint64},
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
        if let Instructions::Number(num) = instruction {
            match num.num_type {
                Uint32 => {
                    let mut v = (num.value.u32() - 2_u32.pow(31)) as i32;
                    v = v - 2_i32.pow(30) - 2_i32.pow(30);
                    Number::i32(Some(v))
                }
                Uint64 => {
                    let v = num.value.u64() - 2_u64.pow(31) - 2_u64.pow(31);
                    Number::i64(Some(v as i64))
                }
                _ => num,
            }
        } else {
            panic!("stack top is not value: {:?}", instruction)
        }
    }

    pub fn pop_all_from_label(&mut self, label_idx: usize) {
        self.stack = self.stack[0..label_idx].to_vec();
        self.label_positions = self.label_positions[0..self.label_positions.len() - 1].to_vec();
    }

    pub fn pop_last_label(&mut self) {
        let label_idx = *self
            .label_positions
            .last()
            .unwrap_or_else(|| panic!("label_positions に値が存在しません"));
        self.stack.swap_remove(label_idx);
    }

    pub fn pop_current_frame(&mut self) {
        let frame_idx = *self
            .frame_positions
            .last()
            .unwrap_or_else(|| panic!("frame_positions に値が存在しません"));
        self.stack = self.stack[0..frame_idx].to_vec();
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

    pub fn label_position(&self, label_idx: usize) -> usize {
        self.label_positions[self.label_positions.len() - label_idx - 1]
    }

    pub fn get_label(&self, label_idx: usize) -> Block {
        let label = if let Instructions::Block(block) = &self.stack[self.label_position(label_idx)]
        {
            block
        } else {
            unreachable!()
        };
        label.clone()
    }

    pub fn inspect(&self) -> String {
        format!(
            "Stack size: {}
#Stack<\n
stack: Vec<Instructions> {:#?}\n
>",
            self.stack.len(),
            self.stack
        )
    }
}
