use crate::{
    instance::{Export, FunctionInstance, Instance},
    node::{BrInstructionNode, ExpressionNode, InstructionNode},
    stack::{Label, LabelType, Number, StackEntry, Value},
    types::{BlockType, NumberType, ValueType},
};

#[derive(Debug, Clone)]
pub struct Frame {
    function: FunctionInstance,
    locals: Vec<Option<Value>>,
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
                ip: 0,
            }
        } else {
            Frame {
                function,
                locals: vec![None; local_count],
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
    instance: Instance,
    frames: Vec<Frame>,
    frame_index: usize,
    stack: Vec<StackEntry>,
    sp: usize,
    depth: usize,
    label_positions: Vec<usize>,
    control_instructions: Vec<InstructionNode>,
}

impl Runtime {
    pub fn new(instance: Instance) -> Self {
        Self {
            instance,
            frames: vec![],
            frame_index: 0,
            stack: vec![],
            sp: 0,
            depth: 0,
            label_positions: vec![],
            control_instructions: vec![],
        }
    }

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
        self.stack
            .pop()
            .unwrap_or_else(|| panic!("No stack entry to pop"))
    }

    fn pop_result(&mut self) -> Option<StackEntry> {
        if self.sp > 0 {
            Some(self.pop_stack())
        } else {
            None
        }
    }

    fn push_label(&mut self, label_type: LabelType, arity: BlockType, size: u32) {
        self.push_stack(StackEntry::label(Label {
            label_type,
            arity,
            size: size as usize,
        }));
        self.label_positions.push(self.sp - 1);
    }

    fn pop_label(&mut self) {
        let label_idx = self
            .label_positions
            .pop()
            .unwrap_or_else(|| panic!("No label to pop"));
        let _ = self.stack.split_off(label_idx);
        self.sp = label_idx;
    }

    fn get_label(&mut self, depth: usize) -> Label {
        self.label_positions.reverse();
        let label_idx = self.label_positions[depth];
        self.label_positions.reverse();

        match &self.stack[label_idx] {
            StackEntry::label(label) => label.clone(),
            _ => panic!("Stack entry is not a label"),
        }
    }

    pub fn execute(&mut self, name: &String, args: Option<Vec<Value>>) -> Option<Number> {
        let export = &self.instance.export_map.get(name).unwrap();
        match export {
            Export::Function { index, name: _ } => {
                let function = self.instance.functions[*index].clone();
                self.push_frame(function.clone(), args);
            }
        };

        while !self.frame_is_empty() {
            self.call();
        }
        let entry = self.pop_result();
        match entry {
            Some(entry) => match entry {
                StackEntry::value(value) => match value {
                    Value::num(number) => Some(number),
                },
                _ => panic!("result must be value"),
            },
            None => None,
        }
    }

    pub fn call(&mut self) {
        let mut frame = self.current_frame();
        while frame.ip < frame.function.code.body.len() {
            let instruction = frame.next_instruction();
            self.invoke(&mut frame, &instruction);
        }
        self.pop_frame();
    }

    pub fn invoke(&mut self, frame: &mut Frame, instruction: &InstructionNode) {
        match instruction {
            InstructionNode::I32Const(node) => {
                self.push_stack(StackEntry::value(Value::num(Number::i32(node.value))));
            }
            InstructionNode::Block(node) => {
                self.push_label(LabelType::Block, node.block_type, node.size);
                self.expression(frame, &node.expr);
                self.cleanup_block_label(node.block_type);
            }
            InstructionNode::Loop(node) => {
                self.push_label(LabelType::Loop, node.block_type, node.size);
                loop {
                    self.expression(frame, &node.expr);
                    if self.control_instructions.len() > 0 {
                        let control_instruction = self.control_instructions.pop().unwrap();
                        match control_instruction {
                            InstructionNode::Br(br_node) => {
                                if br_node.depth > 0 {
                                    self.cleanup_block_label(node.block_type);
                                    break;
                                }
                            }
                            _ => todo!("unimplemented control instruction"),
                        }
                    }
                }
            }
            InstructionNode::If(node) => {
                let condition = self.pop_stack();
                if let StackEntry::value(Value::num(Number::i32(value))) = condition {
                    if value != 0 {
                        self.push_label(LabelType::If, node.block_type, node.size);
                        self.expression(frame, &node.then_expr);
                        self.cleanup_block_label(node.block_type);
                    } else if let Some(else_) = node.else_expr.clone() {
                        self.push_label(LabelType::If, node.block_type, node.size);
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
            InstructionNode::Br(node) => {
                self.control_instructions
                    .push(InstructionNode::Br(BrInstructionNode::new(node.depth)));
            }
            InstructionNode::BrIf(node) => {
                let condition = self.pop_stack();
                if let StackEntry::value(Value::num(Number::i32(value))) = condition {
                    if value != 0 {
                        self.control_instructions
                            .push(InstructionNode::Br(BrInstructionNode::new(node.depth)));
                    }
                } else {
                    panic!("br_if condition must be i32");
                }
            }
            InstructionNode::Call(node) => {
                let function = self.instance.functions[node.function_index as usize].clone();
                let mut args: Vec<Value> = vec![];
                function
                    .function_type
                    .params
                    .val_types
                    .iter()
                    .for_each(|_| {
                        let entry = self.pop_stack();
                        if let StackEntry::value(v) = entry {
                            args.push(v);
                        }
                    });

                self.push_frame(function, Some(args));
                self.call();
            }
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
                let rhs = self.pop_stack();
                let lhs = self.pop_stack();
                match (rhs, lhs) {
                    (StackEntry::value(Value::num(a)), StackEntry::value(Value::num(b))) => {
                        self.push_stack(StackEntry::value(Value::num(a + b)));
                    }
                    _ => panic!("i32.add must have two i32 values on the stack"),
                }
            }
            InstructionNode::I32Sub(_) => {
                let rhs = self.pop_stack();
                let lhs = self.pop_stack();
                match (rhs, lhs) {
                    (StackEntry::value(Value::num(rhs)), StackEntry::value(Value::num(lhs))) => {
                        self.push_stack(StackEntry::value(Value::num(lhs - rhs)));
                    }
                    _ => panic!("i32.sub must have two i32 values on the stack"),
                }
            }
            InstructionNode::I32GeS(_) => {
                let rhs = self.pop_stack();
                let lhs = self.pop_stack();
                match (rhs, lhs) {
                    (StackEntry::value(Value::num(rhs)), StackEntry::value(Value::num(lhs))) => {
                        self.push_stack(StackEntry::value(Value::num(if lhs >= rhs {
                            Number::i32(1)
                        } else {
                            Number::i32(0)
                        })));
                    }
                    _ => panic!("i32.ge_s must have two i32 values on the stack"),
                }
            } // InstructionNode::I32Mul => {
            //     let rhs = frame.pop_u32();
            //     let lhs = frame.pop_u32();
            //     frame.push_u32(a * b);
            // }
            // InstructionNode::I32DivS => {
            //     let rhs = frame.pop_u32();
            //     let lhs = frame.pop_u32();
            //     frame.push_u32(a / b);
            // }
            // InstructionNode::I32DivU => {
            //     let rhs = frame.pop_u32();
            //     let lhs = frame.pop_u32();
            //     frame.push_u32(a / b);
            // }
            InstructionNode::I32RemS(_) => {
                let rhs = self.pop_stack();
                let lhs = self.pop_stack();
                match (rhs, lhs) {
                    (StackEntry::value(Value::num(rhs)), StackEntry::value(Value::num(lhs))) => {
                        self.push_stack(StackEntry::value(Value::num(lhs % rhs)));
                    }
                    _ => panic!("i32.rem_s must have two i32 values on the stack"),
                }
            } // InstructionNode::I32RemU => {
              //     let rhs = frame.pop_u32();
              //     let lhs = frame.pop_u32();
              //     frame.push_u32(a % b);
              // }
              // InstructionNode::I32And => {
              //     let rhs = frame.pop_u32();
              //     let lhs = frame.pop_u32();
              //     frame.push_u32(a & b);
              // }
              // InstructionNode::I32Or => {
              //     let rhs = frame.pop_u32();
              //     let lhs = frame.pop_u32();
              //     frame.push_u32(a | b);
              // }
              // InstructionNode::I32Xor => {
              //     let rhs = frame.pop_u32();
              //     let lhs = frame.pop_u32();
              //     frame.push_u32(a ^ b);
              // }
              // InstructionNode::I32Shl => {
              //     let rhs = frame.pop_u32();
              //     let lhs = frame.pop_u32();
              //     frame.push_u32(a << b);
              // }
              // InstructionNode::I32ShrS => {
              //     let rhs = frame.pop_u32();
              //     let lhs = frame.pop_u32();
              //     frame.push_u32(a >> b);
              // }
        };
    }

    pub fn expression(&mut self, frame: &mut Frame, expr: &ExpressionNode) {
        for instruction in expr.instructions.iter() {
            self.invoke(frame, instruction);
            if self.control_instructions.len() > 0 {
                break;
            }
        }
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
