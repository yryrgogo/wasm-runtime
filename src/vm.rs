use crate::{
    instance::{Export, Function, Instance},
    node::InstructionNode,
};

#[derive(Debug, Clone)]
struct Frame {
    function: Function,
    base_pointer: usize,
    stack: Vec<u64>,
    sp: usize,
}

impl Frame {
    fn new(function: Function) -> Self {
        Self {
            function,
            base_pointer: 0,
            stack: vec![],
            sp: 0,
        }
    }

    fn next_instruction(&mut self) -> &InstructionNode {
        self.sp += 1;
        &self.function.instructions[self.sp - 1]
    }

    fn push(&mut self, value: u64) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> u64 {
        self.stack.pop().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct VM {
    frames: Vec<Frame>,
    frame_index: usize,
    depth: usize,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            frames: vec![],
            frame_index: 0,
            depth: 0,
        }
    }
}

impl VM {
    fn push_frame(&mut self, function: Function) {
        self.frames.push(Frame::new(function));
        self.frame_index += 1;
    }

    fn pop_frame(&mut self) {
        self.frame_index -= 1;
        self.frames.pop();
    }

    fn current_frame(&mut self) -> &mut Frame {
        &mut self.frames[self.frame_index]
    }

    pub fn run(&mut self, instance: &Instance, name: &String) {
        let export = instance.exportMap.get(name).unwrap();
        if let Export::Function { index, name } = export {
            let function = &instance.functions[*index];
            self.push_frame(function.clone());
        } else {
            panic!("cannot run non-function export");
        };

        let frame = self.current_frame();
        frame;

        // loop {
        // let instruction = frame.next_instruction();
        // match instruction {
        //     InstructionNode::I32Const(node) => {
        //         frame.push(node.value);
        //     }
        //     InstructionNode::I32Add => {
        //         let a = frame.pop_u32();
        //         let b = frame.pop_u32();
        //         frame.push_u32(a + b);
        //     }
        //     InstructionNode::I32Sub => {
        //         let a = frame.pop_u32();
        //         let b = frame.pop_u32();
        //         frame.push_u32(a - b);
        //     }
        //     InstructionNode::I32Mul => {
        //         let a = frame.pop_u32();
        //         let b = frame.pop_u32();
        //         frame.push_u32(a * b);
        //     }
        //     InstructionNode::I32DivS => {
        //         let a = frame.pop_u32();
        //         let b = frame.pop_u32();
        //         frame.push_u32(a / b);
        //     }
        //     InstructionNode::I32DivU => {
        //         let a = frame.pop_u32();
        //         let b = frame.pop_u32();
        //         frame.push_u32(a / b);
        //     }
        //     InstructionNode::I32RemS => {
        //         let a = frame.pop_u32();
        //         let b = frame.pop_u32();
        //         frame.push_u32(a % b);
        //     }
        //     InstructionNode::I32RemU => {
        //         let a = frame.pop_u32();
        //         let b = frame.pop_u32();
        //         frame.push_u32(a % b);
        //     }
        //     InstructionNode::I32And => {
        //         let a = frame.pop_u32();
        //         let b = frame.pop_u32();
        //         frame.push_u32(a & b);
        //     }
        //     InstructionNode::I32Or => {
        //         let a = frame.pop_u32();
        //         let b = frame.pop_u32();
        //         frame.push_u32(a | b);
        //     }
        //     InstructionNode::I32Xor => {
        //         let a = frame.pop_u32();
        //         let b = frame.pop_u32();
        //         frame.push_u32(a ^ b);
        //     }
        //     InstructionNode::I32Shl => {
        //         let a = frame.pop_u32();
        //         let b = frame.pop_u32();
        //         frame.push_u32(a << b);
        //     }
        //     InstructionNode::I32ShrS => {
        //         let a = frame.pop_u32();
        //         let b = frame.pop_u32();
        //         frame.push_u32(a >> b);
        //     }
        // }
        // }
    }
}
