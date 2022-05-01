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
        for num in args {
            self.stack.push_values(num);
        }

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

        let result = self.stack.pop_value();
        println!("#[invoke] Result: {:#?}", result);
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
                _ => unreachable!(),
            };
        }
        args.reverse();
        self.stack.push_frame(Frame::new(func, args))
    }

    fn execute(&mut self, frame: &mut Frame) {
        loop {
            match self.stack.next_opcode(frame) {
                Some(0x02) => self.operate_block(frame),
                Some(0x03) => self.operate_block(frame),
                Some(0x04) => self.operate_if(frame),
                Some(0x0b) => self.operate_end(frame),
                Some(0x0c) => self.operate_br(frame),
                Some(0x0d) => self.operate_br_if(frame),
                Some(0x20) => self.operate_local_get(frame),
                Some(0x21) => self.operate_local_set(frame),
                Some(0x22) => self.operate_local_tee(frame),
                Some(0x40) => self.operate_grow_memory(),
                Some(0x41) => self.operate_i32_const(frame),
                Some(0x4f) => self.operate_i32_ge_u(),
                Some(0x6A) => self.operate_i32_add(),
                Some(opcode) => {
                    todo!("#[execute] opcode: {:x}", opcode);
                }
                None => break,
            }
        }
    }

    // 0x02
    fn operate_block(&mut self, frame: &mut Frame) {
        let block_start_counter = frame.get_counter() - 1;

        let label = (*frame
            .function
            .blocks
            .get(&block_start_counter)
            .unwrap_or_else(|| panic!("# [operate_block Label の取得に失敗しました。]")))
        .clone();

        // start_idx は 0x02 オペコードを指しており、次は arity のため2つ飛ばす
        frame.set_counter(label.start_idx + 2);
        println!("# [operate_block] Label {:?}", label);
        self.stack.push_label(label);
    }

    // 0x04
    fn operate_if(&mut self, frame: &mut Frame) {
        let num = self.stack.pop_value();
        if num.value.i32() == 0 {
            let block_start_counter = frame.get_counter() - 1;
            let label = (*frame
                .function
                .blocks
                .get(&block_start_counter)
                .unwrap_or_else(|| panic!("# [operate_if] Label の取得に失敗しました。")))
            .clone();

            frame.set_counter(label.end_idx + 1);
        } else {
            self.operate_block(frame);
        }

        println!("[if] {:?}", num);
    }

    // 0x0b
    fn operate_end(&mut self, frame: &Frame) {
        let counter = frame.get_counter();
        let last_idx = frame.function.expressions.len();
        if counter != last_idx {
            self.stack.pop_last_label();
            return;
        }

        let result = self.stack.pop_value();
        println!("#[operate_end] Result: {:#?}", result);
        if let crate::instructions::Instructions::Frame(_) = self.stack.stack.last().unwrap() {
            self.stack.pop_current_frame();
            self.stack.push_values(result);
        } else {
            unreachable!("#[operate_end] Stack top が Frame ではありません。")
        };

        println!("[operate_end] End");
    }

    // 0x0c
    fn operate_br(&mut self, frame: &mut Frame) {
        let label_idx = self.read_u_leb128(frame);
        let label = self.stack.get_label(label_idx);

        let mut result: Option<Number> = None;
        if label.arity.len() != 0 {
            result = Some(self.stack.pop_value());
        }

        self.stack
            .pop_all_from_label(self.stack.label_position(label_idx));
        if label.instruction == 0x03 {
            frame.set_counter(label.start_idx);
        } else {
            frame.set_counter(label.end_idx + 1);
        }

        if let Some(num) = result {
            self.stack.push_values(num);
        }
    }

    // 0x0d
    fn operate_br_if(&mut self, frame: &mut Frame) {
        let value = self.stack.pop_value().value.i32();

        if value == 0 {
            self.read_u_leb128(frame);
        } else {
            self.operate_br(frame);
        }
    }

    // 0x20
    fn operate_local_get(&mut self, frame: &mut Frame) {
        let local_idx = self.read_u_leb128(frame);
        let local_var = frame.reference_local_var(local_idx as usize);

        println!("[local_get] {:?}", local_var);

        self.stack.push_values(local_var);
    }

    // 0x21
    fn operate_local_set(&mut self, frame: &mut Frame) {
        let local_idx = self.read_u_leb128(frame);
        frame.local_vars[local_idx] = self.stack.pop_value();

        println!("[local_set] {:?}", frame.local_vars[local_idx]);
    }

    // 0x22
    fn operate_local_tee(&mut self, frame: &mut Frame) {
        let local_idx = self.read_u_leb128(frame);
        frame.local_vars[local_idx] = self.stack.peek();

        println!("[local_tee] {:?}", frame.local_vars[local_idx]);
    }

    // 0x40
    fn operate_grow_memory(&mut self) {
        todo!("");
        // let size = self.read_u_leb128(frame);
        // TODO:
        // println!("# [operate_grow_memory]grow {} memory", size)
    }

    // 0x41
    fn operate_i32_const(&mut self, frame: &mut Frame) {
        let value = self.read_s_leb128(frame);
        self.stack.push_values(Number::i32(Some(value as i32)));

        println!("[i32_const] {:?}", Number::i32(Some(value as i32)));
    }

    // 0x4f
    fn operate_i32_ge_u(&mut self) {
        let n2 = self.stack.pop_value();
        let n1 = self.stack.pop_value();
        let result: Number;
        if n1.value > n2.value {
            result = Number::i32(Some(1));
        } else {
            result = Number::i32(Some(0));
        }

        println!("[i32_ge_u] {:?}", result);
        self.stack.push_values(result);
    }

    // 0x6A
    fn operate_i32_add(&mut self) {
        let n2 = self.stack.pop_value();
        let n1 = self.stack.pop_value();
        let n: i32 = n1.value.i32() + n2.value.i32();
        self.stack.push_values(Number::i32(Some(n)));
        println!("[i32_add] {:?}", Number::i32(Some(n)));
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
