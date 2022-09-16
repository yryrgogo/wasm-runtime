use std::env;

use crate::{
    runtime::Runtime,
    stack::{Number, Value},
};

mod buffer;
mod instance;
mod instruction;
mod leb128;
mod module;
mod node;
mod parser;
mod runtime;
mod stack;
mod types;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("file path: {:?}", *file_path);

    let mut bytes = std::fs::read(file_path).expect("file not found");

    let parser = parser::Parser::new().unwrap();
    let mut module = parser.parse(&mut bytes).expect("Failed to parse");

    module.emit();
    // println!("Successfully parse module\n{:#?}", module);
    // println!("emit wasm module\n{:#?}", module.buffer);

    let instance = instance::Instance::new(&mut module);
    let mut runtime = Runtime::default();
    let keys = instance
        .exportMap
        .keys()
        .map(|k| k.to_string())
        .collect::<Vec<String>>();

    let module_args = args[2..]
        .iter()
        .map(|s| Value::num(Number::i32(s.parse::<i32>().unwrap())))
        .collect::<Vec<Value>>();
    let result = runtime.execute(&instance, &keys[0], Some(module_args));
    println!("{:#?}", runtime);
    println!("result: {:#?}", result);
}

#[cfg(test)]
mod parser_tests {
    use crate::node::ExportTypeNode;

    use super::*;

    #[test]
    fn parse_const_i32_module() {
        let file_path = "test/fixtures/const_i32.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let module = parser.parse(&mut bytes).expect("Failed to parse");

        let type_section_function_types = &module.type_section().unwrap().function_types;
        assert_eq!(type_section_function_types[0].params.val_types.len(), 0);
        assert_eq!(type_section_function_types[0].returns.val_types.len(), 1);

        assert_eq!(module.function_section().unwrap().type_indexes.len(), 1);

        let code_section_bodies = &module.code_section().unwrap().bodies;
        assert_eq!(code_section_bodies.len(), 1);
        assert_eq!(code_section_bodies[0].locals.len(), 0);
        assert_eq!(code_section_bodies[0].local_count, 0);
        assert_eq!(code_section_bodies[0].function_body_size, 4);
        assert_eq!(code_section_bodies[0].expr.instructions.len(), 2);
    }

    #[test]
    fn parse_local_i32_get_set_module() {
        let file_path = "test/fixtures/local_i32_var.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let module = parser.parse(&mut bytes).expect("Failed to parse");

        let type_section_function_types = &module.type_section().unwrap().function_types;
        assert_eq!(type_section_function_types[0].params.val_types.len(), 0);
        assert_eq!(type_section_function_types[0].returns.val_types.len(), 1);

        assert_eq!(module.function_section().unwrap().type_indexes.len(), 1);

        let code_section_bodies = &module.code_section().unwrap().bodies;
        assert_eq!(code_section_bodies.len(), 1);
        assert_eq!(code_section_bodies[0].locals.len(), 1);
        assert_eq!(code_section_bodies[0].locals[0].count, 1);
        assert_eq!(code_section_bodies[0].local_count, 1);
        assert_eq!(code_section_bodies[0].function_body_size, 10);
        assert_eq!(code_section_bodies[0].expr.instructions.len(), 4);
    }

    #[test]
    fn parse_local_i32_add_module() {
        let file_path = "test/fixtures/i32_add.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let module = parser.parse(&mut bytes).expect("Failed to parse");

        let type_section_function_types = &module.type_section().unwrap().function_types;
        assert_eq!(type_section_function_types[0].params.val_types.len(), 2);
        assert_eq!(type_section_function_types[0].returns.val_types.len(), 1);

        assert_eq!(module.function_section().unwrap().type_indexes.len(), 1);

        let export_section_exports = &module.export_section().unwrap().exports;
        assert_eq!(export_section_exports.len(), 1);
        assert_eq!(export_section_exports[0].name, "i32_add");
        assert_eq!(export_section_exports[0].export_desc.index, 0);
        assert_eq!(
            export_section_exports[0].export_desc.export_type,
            ExportTypeNode::Function
        );

        let code_section_bodies = &module.code_section().unwrap().bodies;
        assert_eq!(code_section_bodies.len(), 1);
        assert_eq!(code_section_bodies[0].locals.len(), 0);
        assert_eq!(code_section_bodies[0].local_count, 0);
        assert_eq!(code_section_bodies[0].function_body_size, 7);
        assert_eq!(code_section_bodies[0].expr.instructions.len(), 4);
    }

    #[test]
    fn parse_if_else_module() {
        let file_path = "test/fixtures/if_i32_ge_s.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let module = parser.parse(&mut bytes).expect("Failed to parse");

        let type_section_function_types = &module.type_section().unwrap().function_types;
        assert_eq!(type_section_function_types[0].params.val_types.len(), 1);
        assert_eq!(type_section_function_types[0].returns.val_types.len(), 1);

        assert_eq!(module.function_section().unwrap().type_indexes, [0]);

        let export_section_exports = &module.export_section().unwrap().exports;
        assert_eq!(export_section_exports.len(), 1);
        assert_eq!(export_section_exports[0].name, "if_i32_ge_s");
        assert_eq!(export_section_exports[0].export_desc.index, 0);
        assert_eq!(
            export_section_exports[0].export_desc.export_type,
            ExportTypeNode::Function
        );

        let code_section_bodies = &module.code_section().unwrap().bodies;
        assert_eq!(code_section_bodies.len(), 1);
        assert_eq!(code_section_bodies[0].locals.len(), 0);
        assert_eq!(code_section_bodies[0].local_count, 0);
        assert_eq!(code_section_bodies[0].function_body_size, 15);
        assert_eq!(code_section_bodies[0].expr.instructions.len(), 5);
    }

    #[test]
    fn parse_loop_module() {
        let file_path = "test/fixtures/loop.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let module = parser.parse(&mut bytes).expect("Failed to parse");

        let type_section_function_types = &module.type_section().unwrap().function_types;
        assert_eq!(type_section_function_types[0].params.val_types.len(), 0);
        assert_eq!(type_section_function_types[0].returns.val_types.len(), 1);

        assert_eq!(module.function_section().unwrap().type_indexes, [0]);

        let export_section_exports = &module.export_section().unwrap().exports;
        assert_eq!(export_section_exports.len(), 1);
        assert_eq!(export_section_exports[0].name, "loop");
        assert_eq!(export_section_exports[0].export_desc.index, 0);
        assert_eq!(
            export_section_exports[0].export_desc.export_type,
            ExportTypeNode::Function
        );

        let code_section_bodies = &module.code_section().unwrap().bodies;
        assert_eq!(code_section_bodies.len(), 1);
        assert_eq!(code_section_bodies[0].locals.len(), 1);
        assert_eq!(code_section_bodies[0].local_count, 1);
        assert_eq!(code_section_bodies[0].function_body_size, 43);
        match &code_section_bodies[0].expr.instructions[4] {
            node::InstructionNode::Block(block_node) => {
                assert_eq!(block_node.block_type, types::BlockType::Empty);
                assert_eq!(block_node.expr.instructions.len(), 2);
                match &block_node.expr.instructions[0] {
                    node::InstructionNode::Loop(loop_node) => {
                        assert_eq!(loop_node.block_type, types::BlockType::Empty);
                        assert_eq!(loop_node.expr.instructions.len(), 14);
                    }
                    _ => panic!("Expected loop node"),
                }
            }
            _ => panic!("Expected block node"),
        }
        assert_eq!(code_section_bodies[0].expr.instructions.len(), 7);
    }

    #[test]
    fn parse_increment_module() {
        let file_path = "test/fixtures/increment.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let module = parser.parse(&mut bytes).expect("Failed to parse");

        let type_section_function_types = &module.type_section().unwrap().function_types;
        assert_eq!(type_section_function_types.len(), 2);
        assert_eq!(type_section_function_types[0].params.val_types.len(), 2);
        assert_eq!(type_section_function_types[0].returns.val_types.len(), 1);
        assert_eq!(type_section_function_types[1].params.val_types.len(), 1);
        assert_eq!(type_section_function_types[1].returns.val_types.len(), 1);

        assert_eq!(module.function_section().unwrap().type_indexes, [0, 1]);

        let export_section_exports = &module.export_section().unwrap().exports;
        assert_eq!(export_section_exports.len(), 1);
        assert_eq!(export_section_exports[0].name, "increment");
        assert_eq!(export_section_exports[0].export_desc.index, 1);
        assert_eq!(
            export_section_exports[0].export_desc.export_type,
            ExportTypeNode::Function
        );

        let code_section_bodies = &module.code_section().unwrap().bodies;
        assert_eq!(code_section_bodies.len(), 2);
        assert_eq!(code_section_bodies[0].locals.len(), 0);
        assert_eq!(code_section_bodies[0].local_count, 0);
        assert_eq!(code_section_bodies[0].function_body_size, 7);
        assert_eq!(code_section_bodies[0].expr.instructions.len(), 4);
        assert_eq!(code_section_bodies[1].locals.len(), 0);
        assert_eq!(code_section_bodies[1].local_count, 0);
        assert_eq!(code_section_bodies[1].function_body_size, 8);
        assert_eq!(code_section_bodies[1].expr.instructions.len(), 4);
    }

    #[test]
    fn emit_module() {
        let dir = "test/fixtures";
        for file in std::fs::read_dir(dir).unwrap() {
            let file_path = file.unwrap().path().to_str().unwrap().to_string();
            if (&file_path).ends_with(".wasm") {
                let mut bytes = std::fs::read(&file_path).expect("file not found");
                let original_bytes = bytes.clone();
                let parser = parser::Parser::new().unwrap();
                let mut module = parser.parse(&mut bytes).expect("Failed to parse");
                module.emit();

                assert_eq!(module.buffer.bytes, original_bytes);
            }
        }
    }
}

#[cfg(test)]
mod module_node_convert_tests {
    use crate::{
        node::{I32SubInstructionNode, InstructionNode},
        parser,
    };

    #[test]
    fn convert_add_instruction_to_sub() {
        let mut bytes = std::fs::read("test/fixtures/i32_add.wasm").expect("file not found");
        let sub_bytes = std::fs::read("test/fixtures/i32_sub.wasm").expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let mut module = parser.parse(&mut bytes).expect("Failed to parse");

        let mut export_section = module
            .export_section()
            .unwrap_or_else(|| panic!("Expected export section"))
            .clone();
        export_section.update_export_function_name(0, "i32_sub".to_string());
        module.set_export_section(export_section);

        let mut code_section = module
            .code_section()
            .unwrap_or_else(|| panic!("Expected code section"))
            .clone();

        for code in code_section.bodies.iter_mut() {
            code.expr
                .update_instruction(2, InstructionNode::I32Sub(I32SubInstructionNode::default()));
        }
        module.set_code_section(code_section);

        module.emit();

        assert_eq!(module.buffer.bytes, sub_bytes);
    }
}

#[cfg(test)]
mod runtime_tests {
    use crate::{
        instance, parser,
        runtime::Runtime,
        stack::{Number, Value},
    };

    #[test]
    fn run_i32_const() {
        let file_path = "test/fixtures/const_i32.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let mut module = parser.parse(&mut bytes).expect("Failed to parse");
        module.make();

        let instance = instance::Instance::new(&mut module);
        let mut vm = Runtime::default();
        let keys = instance
            .exportMap
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<String>>();
        let result = vm.execute(&instance, &keys[0], None);

        assert_eq!(result, Some(Number::i32(42)));
    }

    #[test]
    fn run_i32_local_get_set() {
        let file_path = "test/fixtures/local_i32_var.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let mut module = parser.parse(&mut bytes).expect("Failed to parse");
        module.make();

        let instance = instance::Instance::new(&mut module);
        let mut vm = Runtime::default();
        let keys = instance
            .exportMap
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<String>>();
        let result = vm.execute(&instance, &keys[0], None);

        assert_eq!(result, Some(Number::i32(55)));
    }

    #[test]
    fn run_i32_add() {
        let file_path = "test/fixtures/i32_add.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let mut module = parser.parse(&mut bytes).expect("Failed to parse");
        module.make();

        let instance = instance::Instance::new(&mut module);
        let mut vm = Runtime::default();
        let keys = instance
            .exportMap
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<String>>();

        let args = vec!["1", "2"]
            .iter()
            .map(|s| Value::num(Number::i32(s.parse::<i32>().unwrap())))
            .collect::<Vec<Value>>();
        let result = vm.execute(&instance, &keys[0], Some(args));

        assert_eq!(result, Some(Number::i32(3)));
    }

    #[test]
    fn run_i32_sub() {
        let file_path = "test/fixtures/i32_sub.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let mut module = parser.parse(&mut bytes).expect("Failed to parse");
        module.make();

        let instance = instance::Instance::new(&mut module);
        let mut vm = Runtime::default();
        let keys = instance
            .exportMap
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<String>>();

        let args = vec!["1", "2"]
            .iter()
            .map(|s| Value::num(Number::i32(s.parse::<i32>().unwrap())))
            .collect::<Vec<Value>>();
        let result = vm.execute(&instance, &keys[0], Some(args));

        assert_eq!(result, Some(Number::i32(-1)));
    }

    #[test]
    fn run_if_then_i32_ge_s() {
        let file_path = "test/fixtures/if_i32_ge_s.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let mut module = parser.parse(&mut bytes).expect("Failed to parse");
        module.make();

        let instance = instance::Instance::new(&mut module);
        let mut vm = Runtime::default();
        let keys = instance
            .exportMap
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<String>>();

        let args = vec!["100"]
            .iter()
            .map(|s| Value::num(Number::i32(s.parse::<i32>().unwrap())))
            .collect::<Vec<Value>>();
        let result = vm.execute(&instance, &keys[0], Some(args));

        assert_eq!(result, Some(Number::i32(1)));
    }

    #[test]
    fn run_if_else_i32_ge_s() {
        let file_path = "test/fixtures/if_i32_ge_s.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let mut module = parser.parse(&mut bytes).expect("Failed to parse");
        module.make();

        let instance = instance::Instance::new(&mut module);
        let mut vm = Runtime::default();
        let keys = instance
            .exportMap
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<String>>();

        let args = vec!["0"]
            .iter()
            .map(|s| Value::num(Number::i32(s.parse::<i32>().unwrap())))
            .collect::<Vec<Value>>();
        let result = vm.execute(&instance, &keys[0], Some(args));

        assert_eq!(result, Some(Number::i32(0)));
    }

    #[test]
    fn run_block() {
        let file_path = "test/fixtures/block.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let mut module = parser.parse(&mut bytes).expect("Failed to parse");
        module.make();

        let instance = instance::Instance::new(&mut module);
        let mut vm = Runtime::default();
        let keys = instance
            .exportMap
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<String>>();

        let result = vm.execute(&instance, &keys[0], None);

        assert_eq!(result, Some(Number::i32(14)));
    }

    #[test]
    fn run_block_no_result() {
        let file_path = "test/fixtures/block_no_result.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let mut module = parser.parse(&mut bytes).expect("Failed to parse");
        module.make();

        let instance = instance::Instance::new(&mut module);
        let mut vm = Runtime::default();
        let keys = instance
            .exportMap
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<String>>();

        let result = vm.execute(&instance, &keys[0], None);

        assert_eq!(result, None);
    }
}
