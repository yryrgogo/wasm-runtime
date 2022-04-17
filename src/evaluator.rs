use crate::module::number::{Number, NumberType};
use crate::structure::frame::Frame;
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

        for (i, _) in func.func_type.parameters.iter().enumerate() {
            let arg_type = func
                .func_type
                .parameters
                .get(func.func_type.parameters.len() - i - 1)
                .unwrap();
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

        self.stack.push_frame(Frame::new(func, args))
    }

    fn execute(&mut self, opcode: &u8, expression: &Vec<u8>, mut counter: usize) -> usize {
        match opcode {
            0x20 => self.execute_local_get(expression, counter),
            0x21 => self.execute_local_set(expression, counter),
            0x22 => self.execute_local_tee(expression, counter),
            _ => {
                todo!("{:x} {:b} {}", opcode, opcode, opcode)
            }
        }
    }

    fn execute_local_get(&mut self, expression: &Vec<u8>, mut counter: usize) -> usize {
        let local_idx = expression.get(counter).unwrap();
        counter += 1;
        let local_var = self
            .stack
            .current_frame()
            .reference_local_var(*local_idx as usize);

        self.stack.push_values(vec![local_var]);
        counter
    }

    fn execute_local_set(&mut self, expression: &Vec<u8>, mut counter: usize) -> usize {
        match read_unsigned_leb128(expression) {
            Ok((local_idx, size)) => {
                counter += size;
                self.stack.current_frame().local_vars[local_idx as usize] = self.stack.pop_value();
                counter
            }
            Err(err) => panic!("Unsigned LEB128 の読み込みに失敗しました。 {}", err),
        }
    }

    fn execute_local_tee(&mut self, expression: &Vec<u8>, mut counter: usize) -> usize {
        match read_unsigned_leb128(expression) {
            Ok((local_idx, size)) => {
                counter += size;
                self.stack.current_frame().local_vars[local_idx as usize] = self.stack.peek();
                counter
            }
            Err(err) => panic!("Unsigned LEB128 の読み込みに失敗しました。 {}", err),
        }
    }

    pub fn invoke(&mut self, func_name: String, args: Vec<Number>) {
        self.stack.push_values(args);
        let func_idx = self.module.exported.get(&func_name).unwrap().index;

        self.call(func_idx);

        let mut counter: usize = 0;

        loop {
            match self.stack.frame_positions.last() {
                Some(_) => {
                    let expression = self.stack.current_expression();
                    let opcode = expression.get(counter).unwrap();
                    counter += 1;
                    counter = self.execute(opcode, &expression, counter);
                }
                None => break,
            }
        }
    }
}
