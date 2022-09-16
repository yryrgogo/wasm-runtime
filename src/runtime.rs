use crate::{
    instance::{Export, FunctionInstance, Instance},
    node::{ExpressionNode, InstructionNode},
    stack::{Label, LabelType, Number, StackEntry, Value},
    types::{BlockType, NumberType, ValueType},
};

#[derive(Debug, Clone)]
pub struct Frame {
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
    label_positions: Vec<usize>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            frames: vec![],
            frame_index: 0,
            stack: vec![],
            sp: 0,
            depth: 0,
            label_positions: vec![],
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

    fn push_stack(&mut self, entry: StackEntry) {
        self.stack.push(entry);
        self.sp += 1;
    }

    fn pop_stack(&mut self) -> StackEntry {
        self.sp -= 1;
        self.stack.pop().unwrap()
    }

    fn pop_result(&mut self) -> Option<StackEntry> {
        Some(self.pop_stack())
    }

    fn push_label(&mut self, label_type: LabelType, arity: BlockType) {
        self.push_stack(StackEntry::label(Label { label_type, arity }));
        self.label_positions.push(self.sp - 1);
    }

    fn pop_label(&mut self) {
        let label_idx = self
            .label_positions
            .pop()
            .unwrap_or_else(|| panic!("No label to pop"));
        let _ = self.stack.split_off(label_idx);
    }

    pub fn execute(
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
                self.invoke(&mut frame, &instruction);
            }
            self.pop_frame();
        }
        let entry = self.pop_result();
        match entry {
            Some(entry) => match entry {
                StackEntry::value(value) => match value {
                    Value::num(number) => Some(number),
                },
                _ => panic!("result must be value"),
            },
            None => todo!("{:#?}", entry),
        }
    }

    pub fn invoke(&mut self, frame: &mut Frame, instruction: &InstructionNode) {
        match instruction {
            InstructionNode::I32Const(node) => {
                self.push_stack(StackEntry::value(Value::num(Number::i32(node.value))));
            }
            InstructionNode::Block(node) => {
                self.push_label(LabelType::Block, node.block_type);
                self.expression(frame, &node.expr);
                self.cleanup_block_label(node.block_type);
            }
            InstructionNode::Loop(node) => {
                self.push_label(LabelType::Loop, node.block_type);
                self.expression(frame, &node.expr);
                self.cleanup_block_label(node.block_type);
            }
            InstructionNode::If(node) => {
                let condition = self.pop_stack();
                if let StackEntry::value(Value::num(Number::i32(value))) = condition {
                    if value != 0 {
                        self.push_label(LabelType::If, node.block_type);
                        self.expression(frame, &node.then_expr);
                        self.cleanup_block_label(node.block_type);
                    } else if let Some(else_) = node.else_expr.clone() {
                        self.push_label(LabelType::If, node.block_type);
                        self.expression(frame, &else_);
                        let result = self.pop_stack();
                        self.pop_label();
                        self.push_stack(result);
                    }
                } else {
                    panic!("if condition must be i32");
                }
            }
            InstructionNode::Else(_) => {}
            InstructionNode::Br(_) => todo!(),
            InstructionNode::BrIf(_) => todo!(),
            InstructionNode::Call(_) => todo!(),
            InstructionNode::End(_) => {}
            InstructionNode::GetLocal(node) => {
                let value = frame.get_local(node.index as usize);
                self.push_stack(StackEntry::value(value.clone().unwrap()));
            }
            InstructionNode::SetLocal(node) => {
                let entry = self.pop_stack();
                match entry {
                    StackEntry::value(v) => {
                        frame.set_local(node.index as usize, v);
                    }
                    _ => panic!("set_local must be value"),
                }
            }
            InstructionNode::I32Add(_) => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                match (a, b) {
                    (StackEntry::value(Value::num(a)), StackEntry::value(Value::num(b))) => {
                        self.push_stack(StackEntry::value(Value::num(a + b)));
                    }
                    _ => panic!("i32.add must have two i32 values on the stack"),
                }
            }
            InstructionNode::I32Sub(_) => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                match (a, b) {
                    (StackEntry::value(Value::num(a)), StackEntry::value(Value::num(b))) => {
                        self.push_stack(StackEntry::value(Value::num(b - a)));
                    }
                    _ => panic!("i32.sub must have two i32 values on the stack"),
                }
            }
            InstructionNode::I32GeS(_) => {
                let a = self.pop_stack();
                let b = self.pop_stack();
                match (a, b) {
                    (StackEntry::value(Value::num(a)), StackEntry::value(Value::num(b))) => {
                        self.push_stack(StackEntry::value(Value::num(if b >= a {
                            Number::i32(1)
                        } else {
                            Number::i32(0)
                        })));
                    }
                    _ => panic!("i32.ge_s must have two i32 values on the stack"),
                }
            } // InstructionNode::I32Mul => {
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

    pub fn expression(&mut self, frame: &mut Frame, expr: &ExpressionNode) {
        expr.instructions.iter().for_each(|instruction| {
            self.invoke(frame, instruction);
        });
    }

    fn cleanup_block_label(&mut self, block_type: BlockType) {
        match block_type {
            BlockType::Empty => {
                self.pop_label();
            }
            BlockType::ValType(ValueType::Number(v)) => {
                let result = self.pop_stack();
                match v {
                    NumberType::I32 => match result.clone() {
                        StackEntry::value(Value::num(n)) => match n {
                            Number::i32(_) => {}
                            _ => unreachable!(),
                        },
                        StackEntry::label(_) => unreachable!(),
                    },
                    NumberType::I64 => match result.clone() {
                        StackEntry::value(Value::num(n)) => match n {
                            Number::i64(_) => {}
                            _ => unreachable!(),
                        },
                        StackEntry::label(_) => unreachable!(),
                    },
                    NumberType::F32 => match result.clone() {
                        StackEntry::value(Value::num(n)) => match n {
                            Number::f32(_) => {}
                            _ => unreachable!(),
                        },
                        StackEntry::label(_) => unreachable!(),
                    },
                    NumberType::F64 => match result.clone() {
                        StackEntry::value(Value::num(n)) => match n {
                            Number::f64(_) => {}
                            _ => unreachable!(),
                        },
                        StackEntry::label(_) => unreachable!(),
                    },
                }
                self.pop_label();
                self.push_stack(result);
            }
        }
    }
}
