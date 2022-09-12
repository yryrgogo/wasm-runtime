use crate::{
    instance::{Export, FunctionInstance, Instance},
    node::InstructionNode,
    stack::{Number, StackEntry, Value},
};

#[derive(Debug, Clone)]
struct Frame {
    function: FunctionInstance,
    locals: Vec<Option<Value>>,
    base_pointer: usize,
    ip: usize,
}

impl Frame {
    fn new(function: FunctionInstance, args: Option<Vec<Value>>) -> Self {
        let local_count =
            function.code.locals.len() + function.function_type.params.val_types.len();

        if let Some(args) = args {
            let mut locals = vec![None; local_count];
            for (i, arg) in args.into_iter().enumerate() {
                locals[i] = Some(arg);
            }
            Frame {
                function,
                locals,
                base_pointer: 0,
                ip: 0,
            }
        } else {
            Frame {
                function,
                locals: vec![None; local_count],
                base_pointer: 0,
                ip: 0,
            }
        }
    }

    fn next_instruction(&mut self) -> InstructionNode {
        self.ip += 1;
        self.function.code.body[self.ip - 1].clone()
    }

    fn get_local(&self, index: usize) -> &Option<Value> {
        &self.locals[index]
    }

    fn set_local(&mut self, index: usize, value: Value) {
        self.locals[index] = Some(value);
    }
}

#[derive(Debug, Clone)]
pub struct Runtime {
    frames: Vec<Frame>,
    frame_index: usize,
    stack: Vec<StackEntry>,
    sp: usize,
    depth: usize,
}

impl Default for Runtime {
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

impl Runtime {
    fn push_frame(&mut self, function: FunctionInstance, args: Option<Vec<Value>>) {
        self.frames.push(Frame::new(function, args));
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

    pub fn invoke(
        &mut self,
        instance: &Instance,
        name: &String,
        args: Option<Vec<Value>>,
    ) -> Option<Number> {
        let export = instance.exportMap.get(name).unwrap();
        if let Export::Function { index, name: _ } = export {
            let function = &instance.functions[*index];
            self.push_frame(function.clone(), args);
        } else {
            panic!("cannot run non-function export");
        };

        let mut frame = self.current_frame();
        while !self.frame_is_empty() {
            while frame.ip < frame.function.code.body.len() {
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
                    InstructionNode::GetLocal(node) => {
                        let value = frame.get_local(node.index as usize);
                        self.stack_push(StackEntry::value(value.clone().unwrap()));
                    }
                    InstructionNode::SetLocal(node) => {
                        let entry = self.stack_pop();
                        match entry {
                            StackEntry::value(v) => {
                                frame.set_local(node.index as usize, v);
                            }
                        }
                    }
                    InstructionNode::I32Add(_) => {
                        let a = self.stack_pop();
                        let b = self.stack_pop();
                        match (a, b) {
                            (
                                StackEntry::value(Value::num(a)),
                                StackEntry::value(Value::num(b)),
                            ) => {
                                self.stack_push(StackEntry::value(Value::num(a + b)));
                            }
                        }
                    }
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
