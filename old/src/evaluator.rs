use crate::module::function::Function;
use crate::module::number::Number;
use crate::structure::frame::Frame;
use crate::util::leb::read_signed_leb128;
use crate::util::leb::read_unsigned_leb128;
use crate::{module::Module, stack::Stack};

pub struct Evaluator {
    stack: Stack,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            stack: Stack::new(),
        }
    }

    pub fn invoke(
        &mut self,
        module: &Module,
        func_name: &String,
        args: Vec<Number>,
    ) -> Option<Number> {
        for num in args {
            self.stack.push_values(num);
        }

        let func_idx = module.exports.get(func_name).unwrap().index;
        println!("Function name: {}, index: {}", func_name, func_idx);
        self.call(module, func_idx);
        self.stack.pop_value()
    }

    fn call(&mut self, module: &Module, func_idx: usize) {
        let func = self.get_function(module, func_idx);
        let args = self.get_args(&func);

        // println!("[call] callee function: {:#?}", func);
        let result_num = func.func_type.results.len();
        self.stack.push_frame(Frame::new(func, args));

        let mut frame = self.stack.current_frame().unwrap();
        self.execute(module, &mut frame);

        let mut results: Vec<Number> = vec![];
        for _ in 0..result_num {
            let result = self.stack.pop_value();
            let n = result.unwrap_or_else(|| panic!("[call] 関数の戻り値が存在しません"));
            results.push(n);
        }
        self.stack.pop_current_frame();
        results.reverse();
        for n in results {
            self.stack.push_values(n);
        }
    }

    fn execute(&mut self, module: &Module, frame: &mut Frame) {
        loop {
            let opcode = self.stack.next_opcode(frame);
            // println!("counter: {}", frame.get_counter());
            // println!("{:#?}", frame);

            match opcode {
                Some(0x02) => self.operate_block(frame),
                Some(0x03) => self.operate_block(frame),
                Some(0x04) => self.operate_if(frame),
                Some(0x05) => self.operate_else(frame),
                Some(0x10) => self.operate_call(module, frame),
                Some(0x0b) => {
                    self.operate_end(frame);
                }
                Some(0x0c) => self.operate_br(frame),
                Some(0x0d) => self.operate_br_if(frame),
                Some(0x0f) => {
                    break;
                }
                Some(0x20) => self.operate_local_get(frame),
                Some(0x21) => self.operate_local_set(frame),
                Some(0x22) => self.operate_local_tee(frame),
                Some(0x23) => self.operate_global_get(module, frame),
                Some(0x40) => self.operate_grow_memory(),
                Some(0x41) => self.operate_i32_const(frame),
                Some(0x42) => self.operate_i64_const(frame),
                Some(0x44) => self.operate_f64_const(frame),
                Some(0x46) => self.operate_i32_eq(),
                Some(0x4f) => self.operate_i32_ge_u(),
                Some(0x6A) => self.operate_i32_add(),
                Some(0x70) => self.operate_i32_rem_u(),
                Some(0x74) => self.operate_i32_shl(),
                Some(0x92) => self.operate_f32_add(),
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
            .unwrap_or_else(|| {
                panic!(
                    "[operate_block] Label の取得に失敗しました。 counter: {}]",
                    block_start_counter
                )
            }))
        .clone();

        // start_idx は 0x02 オペコードを指しており、次は arity のため2つ飛ばす
        frame.set_counter(label.start_idx + 2);
        println!("[operate_block] Label {:?}", label);
        self.stack.push_label(label);
    }

    // 0x04
    fn operate_if(&mut self, frame: &mut Frame) {
        let num = self
            .stack
            .pop_value()
            .unwrap_or_else(|| panic!("[0x04] if の条件値が存在しません。"));
        if num == Number::Int32(0) {
            let block_start_counter = frame.get_counter() - 1;
            let label = (*frame
                .function
                .blocks
                .get(&block_start_counter)
                .unwrap_or_else(|| panic!("[operate_if] Label の取得に失敗しました。")))
            .clone();

            frame.set_counter(label.end_idx + 1);
        } else {
            self.operate_block(frame);
        }
    }

    // 0x05
    fn operate_else(&mut self, frame: &mut Frame) {
        println!("[operate_else] ${:#?}", frame);
    }

    // 0x0b
    fn operate_end(&mut self, frame: &Frame) {
        let counter = frame.get_counter();
        let last_idx = frame.function.bytecodes.len();
        if counter != last_idx {
            self.stack.pop_last_label();
        }
    }

    // 0x0c
    fn operate_br(&mut self, frame: &mut Frame) {
        let label_idx = self.read_u_leb128(frame);
        let label = self.stack.get_label(label_idx);

        let mut result: Option<Number> = None;
        if label.arity.len() != 0 {
            let return_value = self.stack.pop_value().unwrap_or_else(|| {
                panic!(
                    "arity {:#?} に対応する値が Stack に存在しません。",
                    label.arity
                )
            });
            result = Some(return_value);
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
        let value = self.stack.pop_value().unwrap_or_else(|| {
            panic!("[0x0d] br_if の条件式に対応する値が Stack に存在しません。")
        });

        if value == Number::Uint32(0) || value == Number::Int32(0) {
            self.read_u_leb128(frame);
        } else {
            self.operate_br(frame);
        }
    }

    // 0x0f
    fn operate_return(&mut self, frame: &mut Frame) {}

    // 0x10
    fn operate_call(&mut self, module: &Module, frame: &mut Frame) {
        let func_idx = self.read_u_leb128(frame);
        println!("[call] call function index: {}", func_idx);

        // Frame が既に存在する場合、counter を保持するため current_frame を上書きする
        // current_frame は最初に push された frame を clone して返しているため、初期状態のままになっている（&mut で返すようにするのは lifetime 解決が難しく断念）
        self.stack.update_current_frame(frame.clone());

        self.call(module, func_idx);
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
        frame.local_vars[local_idx] = self.stack.pop_value().unwrap_or_else(|| {
            panic!(
                "[0x21] local var {} にセットする値が Stack に存在しません。",
                local_idx
            )
        });

        println!("[local_set] {:?}", frame.local_vars[local_idx]);
    }

    // 0x22
    fn operate_local_tee(&mut self, frame: &mut Frame) {
        let local_idx = self.read_u_leb128(frame);
        frame.local_vars[local_idx] = self.stack.peek();

        println!("[local_tee] {:?}", frame.local_vars[local_idx]);
    }

    // 0x23
    fn operate_global_get(&mut self, module: &Module, frame: &mut Frame) {
        let global_idx = self.read_u_leb128(frame);
        let global_var = module
            .global_vars
            .get(global_idx as usize)
            .unwrap_or_else(|| {
                panic!(
                    "[0x23] global var {} にセットする値が Stack に存在しません。",
                    global_idx
                )
            });

        println!("[global_get] {:?}", global_var);

        self.stack.push_values(global_var.clone());
    }

    // 0x40
    fn operate_grow_memory(&mut self) {
        todo!("");
        // let size = self.read_u_leb128(frame);
        // TODO:
        // println!("[operate_grow_memory]grow {} memory", size)
    }

    // 0x41
    fn operate_i32_const(&mut self, frame: &mut Frame) {
        let value = self.read_s_leb128(frame);
        self.stack.push_values(Number::Int32(value as i32));

        println!("[i32_const] {:?}", Number::Int32(value as i32));
    }

    // 0x42
    fn operate_i64_const(&mut self, frame: &mut Frame) {
        let value = self.read_s_leb128(frame);
        self.stack.push_values(Number::Int64(value as i64));

        println!("[i64_const] {:?}", Number::Int64(value as i64));
    }

    // 0x44
    fn operate_f64_const(&mut self, frame: &mut Frame) {
        let value = self.read_u_leb128(frame);
        self.stack.push_values(Number::Float64(value as u64));

        println!("[f64_const] {:?}", Number::Float64(value as u64));
    }

    // 0x46
    fn operate_i32_eq(&mut self) {
        let right = self
            .stack
            .pop_value()
            .unwrap_or_else(|| panic!("[0x46] 右値が Stack に存在しません。"));
        let left = self
            .stack
            .pop_value()
            .unwrap_or_else(|| panic!("[0x46] 左値が Stack に存在しません。"));
        let result: Number;
        if left == right {
            result = Number::Int32(1);
        } else {
            result = Number::Int32(0);
        }

        println!("[i32_eq] {:?}", result);
        self.stack.push_values(result);
    }

    // 0x4f
    fn operate_i32_ge_u(&mut self) {
        let right = self
            .stack
            .pop_value()
            .unwrap_or_else(|| panic!("[0x4f] 右値が Stack に存在しません。"));
        let left = self
            .stack
            .pop_value()
            .unwrap_or_else(|| panic!("[0x4f] 左値が Stack に存在しません。"));
        let result: Number;
        if left >= right {
            result = Number::Int32(1);
        } else {
            result = Number::Int32(0);
        }

        println!("[i32_ge_u] {:?}", result);
        self.stack.push_values(result);
    }

    // 0x6A
    fn operate_i32_add(&mut self) {
        let right = self
            .stack
            .pop_value()
            .unwrap_or_else(|| panic!("[0x6A] 右値が Stack に存在しません。"));
        let left = self
            .stack
            .pop_value()
            .unwrap_or_else(|| panic!("[0x6A] 左値が Stack に存在しません。"));
        let result = left + right;
        println!("[i32_add] {:?}", result);
        self.stack.push_values(result);
    }

    // 0x70
    fn operate_i32_rem_u(&mut self) {
        let right = self
            .stack
            .pop_value()
            .unwrap_or_else(|| panic!("[0x70] 右値が Stack に存在しません。"));
        let left = self
            .stack
            .pop_value()
            .unwrap_or_else(|| panic!("[0x70] 左値が Stack に存在しません。"));
        let result = left % right;
        println!("[i32_rem_u] {:?}", &result);
        self.stack.push_values(result)
    }

    // 0x74
    fn operate_i32_shl(&mut self) {
        let shift_left = self
            .stack
            .pop_value()
            .unwrap_or_else(|| panic!("[0x74] 左シフトする数が Stack に存在しません。"));
        let value = self
            .stack
            .pop_value()
            .unwrap_or_else(|| panic!("[0x74] 左シフトする値が Stack に存在しません。"));
        self.stack.push_values(value << shift_left)
    }

    // 0x92
    fn operate_f32_add(&mut self) {
        let right = self
            .stack
            .pop_value()
            .unwrap_or_else(|| panic!("[0x92] 左値が Stack に存在しません。"));
        let left = self
            .stack
            .pop_value()
            .unwrap_or_else(|| panic!("[0x92] 右値が Stack に存在しません。"));
        let result = left + right;
        println!("[f32_add] {:?}", &result);
        self.stack.push_values(result);
    }

    fn read_u_leb128(&mut self, frame: &mut Frame) -> usize {
        match read_unsigned_leb128(&self.stack.current_bytecodes(frame)) {
            Ok((value, size)) => {
                frame.increment_counter(size);
                value
            }
            Err(_) => panic!("unsigned leb128 の decode に失敗しました。"),
        }
    }

    fn read_s_leb128(&mut self, frame: &mut Frame) -> isize {
        match read_signed_leb128(&self.stack.current_bytecodes(frame)) {
            Ok((value, size)) => {
                frame.increment_counter(size);
                value
            }
            Err(_) => panic!("signed leb128 の decode に失敗しました。"),
        }
    }

    fn get_function(&self, module: &Module, func_idx: usize) -> Function {
        module
            .functions
            .get(func_idx)
            .unwrap_or_else(|| panic!("not found function index: {}", func_idx))
            .clone()
    }

    fn get_args(&mut self, func: &Function) -> Vec<Number> {
        let mut args: Vec<Number> = vec![];
        for (_, _) in func.func_type.parameters.iter().enumerate() {
            let num = self.stack.pop_value().unwrap_or_else(|| {
                panic!(
                    "
Function のパラメータが Stack に存在しません。
function type: {:#?}
stack: {:#?}
",
                    func.func_type, self.stack
                )
            });

            match num {
                Number::Int32(_) | Number::Int64(_) | Number::Float32(_) | Number::Float64(_) => {
                    args.push(num);
                }
                _ => unreachable!(),
            };
        }
        args.reverse();
        args
    }
}

#[cfg(test)]
mod evaluator_tests {
    use crate::decoder::Decoder;

    use super::*;

    #[test]
    fn can_evaluate_fibonacci() {
        let path = "src/wasm/fibonacci/fib.wasm".to_string();
        let mut decoder = Decoder::new(Some(&path), None).unwrap();

        decoder.run();

        let mut eval = Evaluator::new();

        for func_name in decoder.module.exports.keys() {
            let result = eval.invoke(&decoder.module, &func_name, vec![Number::Int32(3)]);
            assert_eq!(result.unwrap(), Number::Int32(2));

            let result = eval.invoke(&decoder.module, &func_name, vec![Number::Int32(5)]);
            assert_eq!(result.unwrap(), Number::Int32(5));

            let result = eval.invoke(&decoder.module, &func_name, vec![Number::Int32(8)]);
            assert_eq!(result.unwrap(), Number::Int32(21));

            let result = eval.invoke(&decoder.module, &func_name, vec![Number::Int32(10)]);
            assert_eq!(result.unwrap(), Number::Int32(55));

            let result = eval.invoke(&decoder.module, &func_name, vec![Number::Int32(20)]);
            assert_eq!(result.unwrap(), Number::Int32(6765));
        }
    }

    #[test]
    fn can_evaluate_add_i32() {
        let path = "src/wasm/math/addInt.wasm".to_string();
        let mut decoder = Decoder::new(Some(&path), None).unwrap();

        decoder.run();

        let mut eval = Evaluator::new();

        for func_name in decoder.module.exports.keys() {
            let result = eval.invoke(
                &decoder.module,
                &func_name,
                vec![Number::Int32(1), Number::Int32(2)],
            );
            assert_eq!(result.unwrap(), Number::Int32(3));

            let result = eval.invoke(
                &decoder.module,
                &func_name,
                vec![Number::Int32(-1), Number::Int32(2)],
            );
            assert_eq!(result.unwrap(), Number::Int32(1));

            let result = eval.invoke(
                &decoder.module,
                &func_name,
                vec![Number::Int32(1), Number::Int32(99999)],
            );
            assert_eq!(result.unwrap(), Number::Int32(100000));

            let result = eval.invoke(
                &decoder.module,
                &func_name,
                vec![Number::Int32(99999999), Number::Int32(99999)],
            );
            assert_eq!(result.unwrap(), Number::Int32(100099998));
        }
    }

    // #[test]
    /// TODO: wasm における Float の扱いを調べる
    // fn can_evaluate_add_f32() {
    //     let path = "src/wasm/math/addFloat.wasm".to_string();
    //     let mut decoder = Decoder::new(Some(&path), None).unwrap();

    //     decoder.run();

    //     let mut eval = Evaluator::new();

    //     for func_name in decoder.module.exports.keys() {
    //         let result = eval.invoke(
    //             &decoder.module,
    //             &func_name,
    //             vec![Number::Float32(1.0), Number::Float32(2.0)],
    //         );
    //         assert_eq!(result.unwrap(), Number::Float32(3.0));

    //         let result = eval.invoke(
    //             &decoder.module,
    //             &func_name,
    //             vec![Number::Float32(1.1), Number::Float32(2.2)],
    //         );
    //         // FIXME:
    //         assert_eq!(result.unwrap(), Number::Float32(3.3000002));

    //         let result = eval.invoke(
    //             &decoder.module,
    //             &func_name,
    //             vec![Number::Float32(1.111111), Number::Float32(2.222222)],
    //         );
    //         assert_eq!(result.unwrap(), Number::Float32(3.333333));

    //         let result = eval.invoke(
    //             &decoder.module,
    //             &func_name,
    //             vec![Number::Float32(-1.0), Number::Float32(2.0)],
    //         );
    //         assert_eq!(result.unwrap(), Number::Float32(1.0));

    //         let result = eval.invoke(
    //             &decoder.module,
    //             &func_name,
    //             vec![Number::Float32(1.0), Number::Float32(99999.0)],
    //         );
    //         assert_eq!(result.unwrap(), Number::Float32(100000.0));

    //         let result = eval.invoke(
    //             &decoder.module,
    //             &func_name,
    //             vec![Number::Float32(99999999.0), Number::Float32(99999.0)],
    //         );
    //         assert_eq!(result.unwrap(), Number::Float32(100099998.0));
    //     }
    // }

    #[test]
    fn can_evaluate_twice_int() {
        let path = "src/wasm/math/twice.wasm".to_string();
        let mut decoder = Decoder::new(Some(&path), None).unwrap();

        decoder.run();

        let mut eval = Evaluator::new();

        for func_name in decoder.module.exports.keys() {
            let result = eval.invoke(&decoder.module, &func_name, vec![Number::Int32(1)]);
            assert_eq!(result.unwrap(), Number::Int32(2));

            let result = eval.invoke(&decoder.module, &func_name, vec![Number::Int32(10)]);
            assert_eq!(result.unwrap(), Number::Int32(20));

            let result = eval.invoke(&decoder.module, &func_name, vec![Number::Int32(55)]);
            assert_eq!(result.unwrap(), Number::Int32(110));
        }
    }

    #[test]
    fn can_evaluate_prime_test() {
        let path = "src/wasm/math/isPrime.wasm".to_string();
        let mut decoder = Decoder::new(Some(&path), None).unwrap();

        decoder.run();

        let mut eval = Evaluator::new();

        for func_name in decoder.module.exports.keys() {
            for n in vec![1, 4, 8, 12, 20, 30, 33, 44, 55, 66, 77, 88, 99] {
                let result = eval.invoke(&decoder.module, &func_name, vec![Number::Int32(n)]);
                assert_eq!(result.unwrap(), Number::Int32(0));
            }

            for n in vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97,
            ] {
                let result = eval.invoke(&decoder.module, &func_name, vec![Number::Int32(n)]);
                assert_eq!(result.unwrap(), Number::Int32(1));
            }
        }
    }

    // 外部からインポートしたモジュールをどう wasm で呼ぶのかわからず断念。モジュールの実行バイナリが渡される訳ではなさそうだが？
    // #[test]
    // fn can_evaluate_if_else_simple() {
    //     let path = "src/wasm/if-else/ifElseSimple.wasm".to_string();
    //     let mut decoder = Decoder::new(Some(&path), None).unwrap();

    //     decoder.run();

    //     let mut eval = Evaluator::new();

    //     println!("{:#?}", decoder.module);
    //     for func_name in decoder.module.exports.keys() {
    //         let result = eval.invoke(&decoder.module, &func_name, vec![Number::Int32(0)]);
    //         assert_eq!(result.unwrap(), Number::Int32(100));
    //     }
    // }
}
