use crate::module::number::{Number, NumberType};
use crate::structure::frame::Frame;
use crate::util::leb::read_signed_leb128;
use crate::util::leb::read_unsigned_leb128;
use crate::{module::Module, stack::Stack};

pub struct Evaluator {
    module: Module,
    stack: Stack,
}

impl Evaluator {
    pub fn new(module: Module) -> Evaluator {
        Evaluator {
            stack: Stack::new(),
            module: module,
        }
    }

    pub fn invoke(&mut self, func_name: String, args: Vec<Number>) {
        self.stack.push_values(args);

        let func_idx = self.module.exported.get(&func_name).unwrap().index;
        self.call(func_idx);

        loop {
            match self.stack.current_frame() {
                Some(ref mut f) => {
                    self.execute(f);
                }
                None => break,
            }
        }
    }

    fn call(&mut self, func_idx: usize) {
        let func = self.module.functions.get(func_idx).unwrap().clone();
        let mut args: Vec<Number> = vec![];

        for (_, _) in func.func_type.parameters.iter().enumerate() {
            let value = self.stack.pop_value();
            match value.num_type {
                NumberType::Int32 => {
                    args.push(value);
                }
                NumberType::Int64 => {
                    args.push(value);
                }
                NumberType::Float32 => {
                    args.push(value);
                }
                NumberType::Float64 => {
                    args.push(value);
                }
            };
        }
        args.reverse();
        self.stack.push_frame(Frame::new(func, args))
    }

    fn execute(&mut self, frame: &mut Frame) {
        loop {
            match self.stack.next_opcode(frame) {
                Some(0x20) => self.execute_local_get(frame),
                Some(0x21) => self.execute_local_set(frame),
                Some(0x22) => self.execute_local_tee(frame),
                Some(0x41) => self.execute_i32_const(frame),
                Some(opcode) => {
                    println!("{:?}", self.stack.stack);
                    todo!("opcode: {:x}", opcode);
                }
                None => break,
            }
        }
    }

    fn execute_local_get(&mut self, frame: &mut Frame) {
        let local_idx = self.read_u_leb128(frame);
        let local_var = frame.reference_local_var(local_idx as usize);
        self.stack.push_values(vec![local_var]);
    }

    fn execute_local_set(&mut self, frame: &mut Frame) {
        let local_idx = self.read_u_leb128(frame);
        frame.local_vars[local_idx] = self.stack.pop_value();
    }

    fn execute_local_tee(&mut self, frame: &mut Frame) {
        let local_idx = self.read_u_leb128(frame);
        frame.local_vars[local_idx] = self.stack.peek();
    }

    fn execute_i32_const(&mut self, frame: &mut Frame) {
        let value = self.read_s_leb128(frame);
        self.stack
            .push_values(vec![Number::i32(Some(value as i32))]);
    }

    fn read_u_leb128(&mut self, frame: &mut Frame) -> usize {
        match read_unsigned_leb128(&self.stack.current_expression(frame)) {
            Ok((value, size)) => {
                frame.increment_counter(size);
                value
            }
            Err(_) => panic!("unsigned leb128 の decode に失敗しました。"),
        }
    }

    fn read_s_leb128(&mut self, frame: &mut Frame) -> isize {
        match read_signed_leb128(&self.stack.current_expression(frame)) {
            Ok((value, size)) => {
                frame.increment_counter(size);
                value
            }
            Err(_) => panic!("signed leb128 の decode に失敗しました。"),
        }
    }
}
