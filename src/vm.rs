use crate::{
    instance::{Export, Function, Instance},
    node::InstructionNode,
    stack::{Number, StackEntry, Value},
};

#[derive(Debug, Clone)]
struct Frame {
    function: Function,
    base_pointer: usize,
    ip: usize,
}

impl Frame {
    fn new(function: Function) -> Self {
        Self {
            function,
            base_pointer: 0,
            ip: 0,
        }
    }

    fn next_instruction(&mut self) -> &InstructionNode {
        self.ip += 1;
        &self.function.instructions[self.ip - 1]
    }
}

#[derive(Debug, Clone)]
pub struct VM {
    frames: Vec<Frame>,
    frame_index: usize,
    stack: Vec<StackEntry>,
    sp: usize,
    depth: usize,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            frames: vec![],
            frame_index: 0,
            stack: vec![],
            sp: 0,
            depth: 0,
        }
    }
}

impl VM {
    fn push_frame(&mut self, function: Function) {
        self.frames.push(Frame::new(function));
        self.frame_index += 1;
    }

    fn pop_frame(&mut self) -> Frame {
        self.frame_index -= 1;
        self.frames
            .pop()
            .unwrap_or_else(|| panic!("No frame to pop"))
    }

    fn current_frame(&mut self) -> Frame {
        self.frames[self.frame_index - 1].clone()
    }

    fn frame_is_empty(&self) -> bool {
        self.frame_index == 0
    }

    fn stack_push(&mut self, entry: StackEntry) {
        self.stack.push(entry);
        self.sp += 1;
    }

    fn stack_pop(&mut self) -> StackEntry {
        self.sp -= 1;
        self.stack.pop().unwrap()
    }

    pub fn run(&mut self, instance: &Instance, name: &String) -> Option<Number> {
        let export = instance.exportMap.get(name).unwrap();
        if let Export::Function { index, name: _ } = export {
            let function = &instance.functions[*index];
            self.push_frame(function.clone());
        } else {
            panic!("cannot run non-function export");
        };

        let mut frame = self.current_frame();
        while !self.frame_is_empty() {
            while frame.ip < frame.function.instructions.len() {
                let instruction = frame.next_instruction();
                match instruction {
                    InstructionNode::I32Const(node) => {
                        self.stack_push(StackEntry::value(Value::num(Number::i32(node.value))));
                    }
                    InstructionNode::Block(_) => todo!(),
                    InstructionNode::Loop(_) => todo!(),
                    InstructionNode::If(_) => todo!(),
                    InstructionNode::Else(_) => todo!(),
                    InstructionNode::Br(_) => todo!(),
                    InstructionNode::BrIf(_) => todo!(),
                    InstructionNode::Call(_) => todo!(),
                    InstructionNode::End(_) => {}
                    InstructionNode::GetLocal(_) => todo!(),
                    InstructionNode::SetLocal(_) => todo!(),
                    InstructionNode::I32Add(_) => todo!(),
                    InstructionNode::I32Sub(_) => todo!(),
                    InstructionNode::I32GeS(_) => todo!(),
                    // InstructionNode::I32Add => {
                    //     let a = frame.pop_u32();
                    //     let b = frame.pop_u32();
                    //     frame.push_u32(a + b);
                    // }
                    // InstructionNode::I32Sub => {
                    //     let a = frame.pop_u32();
                    //     let b = frame.pop_u32();
                    //     frame.push_u32(a - b);
                    // }
                    // InstructionNode::I32Mul => {
                    //     let a = frame.pop_u32();
                    //     let b = frame.pop_u32();
                    //     frame.push_u32(a * b);
                    // }
                    // InstructionNode::I32DivS => {
                    //     let a = frame.pop_u32();
                    //     let b = frame.pop_u32();
                    //     frame.push_u32(a / b);
                    // }
                    // InstructionNode::I32DivU => {
                    //     let a = frame.pop_u32();
                    //     let b = frame.pop_u32();
                    //     frame.push_u32(a / b);
                    // }
                    // InstructionNode::I32RemS => {
                    //     let a = frame.pop_u32();
                    //     let b = frame.pop_u32();
                    //     frame.push_u32(a % b);
                    // }
                    // InstructionNode::I32RemU => {
                    //     let a = frame.pop_u32();
                    //     let b = frame.pop_u32();
                    //     frame.push_u32(a % b);
                    // }
                    // InstructionNode::I32And => {
                    //     let a = frame.pop_u32();
                    //     let b = frame.pop_u32();
                    //     frame.push_u32(a & b);
                    // }
                    // InstructionNode::I32Or => {
                    //     let a = frame.pop_u32();
                    //     let b = frame.pop_u32();
                    //     frame.push_u32(a | b);
                    // }
                    // InstructionNode::I32Xor => {
                    //     let a = frame.pop_u32();
                    //     let b = frame.pop_u32();
                    //     frame.push_u32(a ^ b);
                    // }
                    // InstructionNode::I32Shl => {
                    //     let a = frame.pop_u32();
                    //     let b = frame.pop_u32();
                    //     frame.push_u32(a << b);
                    // }
                    // InstructionNode::I32ShrS => {
                    //     let a = frame.pop_u32();
                    //     let b = frame.pop_u32();
                    //     frame.push_u32(a >> b);
                    // }
                }
            }
            self.pop_frame();
        }
        let entry = self.stack_pop();
        match entry {
            StackEntry::value(value) => match value {
                Value::num(number) => Some(number),
            },
        }
    }
}
