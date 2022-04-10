use crate::module::number::{Number, NumberType};
use crate::module::value::Value;
use crate::structure::frame::Frame;
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
            match value {
                Value::Int32(v) => {
                    if *arg_type == NumberType::Int32 {
                        args.push(Number::i32(Some(v)));
                        continue;
                    }
                    panic!(
                        "invalid popped value. Int32 is expected. {:?} {:?}",
                        value, arg_type
                    )
                }
                Value::Int64(v) => {
                    if *arg_type == NumberType::Int64 {
                        args.push(Number::i64(Some(v)));
                        continue;
                    }
                    panic!(
                        "invalid popped value. Int64 is expected. {:?} {:?}",
                        value, arg_type
                    )
                }
                Value::Float32(v) => {
                    if *arg_type == NumberType::Float32 {
                        args.push(Number::f32(Some(v)));
                        continue;
                    }
                    panic!(
                        "invalid popped value. Float32 is expected. {:?} {:?}",
                        value, arg_type
                    )
                }
                Value::Float64(v) => {
                    if *arg_type == NumberType::Float64 {
                        args.push(Number::f64(Some(v)));
                        continue;
                    }
                    panic!(
                        "invalid popped value. Float64 is expected. {:?} {:?}",
                        value, arg_type
                    )
                }
            };
        }

        self.stack.push_frame(Frame::new(func, args))
    }

    fn execute(&self, opcode: &u8) {
        todo!("{:b} {:x}", opcode, opcode)
    }

    pub fn invoke(&mut self, func_name: String, args: Vec<Value>) {
        self.stack.push_values(args);
        let func_idx = self.module.exported.get(&func_name).unwrap().index;

        self.call(func_idx);

        println!("{:?}", self.stack.frame_positions);

        loop {
            let mut counter: usize = 0;
            match self.stack.frame_positions.last() {
                Some(_) => {
                    let expression = self.stack.current_expression();
                    let opcode = expression.get(counter).unwrap();
                    self.execute(opcode);
                    counter += 1;
                }
                None => break,
            }
        }
    }
}
