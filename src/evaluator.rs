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

    fn read_u_leb128(&mut self) -> usize {
        match read_unsigned_leb128(&self.stack.current_expression()) {
            Ok((value, size)) => {
                self.stack.current_frame().increment_counter(size);
                value
            }
            Err(_) => panic!("unsigned leb128 の decode に失敗しました。"),
        }
    }

    fn read_s_leb128(&mut self) -> isize {
        match read_signed_leb128(&self.stack.current_expression()) {
            Ok((value, size)) => {
                self.stack.current_frame().increment_counter(size);
                value
            }
            Err(_) => panic!("signed leb128 の decode に失敗しました。"),
        }
    }

    fn execute(&mut self) {
        let opcode = self.stack.next_opcode();
        match opcode {
            0x20 => self.execute_local_get(),
            0x21 => self.execute_local_set(),
            0x22 => self.execute_local_tee(),
            0x41 => self.execute_i32_const(),
            _ => {
                println!("{:?}", self.stack.stack);
                todo!("{:x}", opcode);
            }
        }
    }

    fn execute_local_get(&mut self) {
        let local_idx = self.read_u_leb128();
        let local_var = self
            .stack
            .current_frame()
            .reference_local_var(local_idx as usize);
        self.stack.push_values(vec![local_var]);
    }

    fn execute_local_set(&mut self) {
        let local_idx = self.read_u_leb128();
        self.stack.current_frame().local_vars[local_idx] = self.stack.pop_value();
    }

    fn execute_local_tee(&mut self) {
        let local_idx = self.read_u_leb128();
        self.stack.current_frame().local_vars[local_idx] = self.stack.peek();
    }

    fn execute_i32_const(&mut self) {
        let value = self.read_s_leb128();
        self.stack
            .push_values(vec![Number::i32(Some(value as i32))]);
    }

    pub fn invoke(&mut self, func_name: String, args: Vec<Number>) {
        self.stack.push_values(args);
        let func_idx = self.module.exported.get(&func_name).unwrap().index;

        self.call(func_idx);

        loop {
            match self.stack.frame_positions.last() {
                Some(_) => {
                    self.execute();
                }
                None => break,
            }
        }
    }
}
