use std::env;

mod buffer;
mod instruction;
mod leb128;
mod module;
mod node;
mod parser;
mod types;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("file path: {:?}", *file_path);

    let mut bytes = std::fs::read(file_path).expect("file not found");

    let parser = parser::Parser::new().unwrap();
    let mut module = parser.parse(&mut bytes).expect("Failed to parse");
    // println!("Successfully parse module\n{:#?}", module);

    module.emit();
    println!("emit wasm module\n{:#?}", module.buffer);
}

#[cfg(test)]
mod parser_tests {
    use crate::node::ExportType;

    use super::*;

    #[test]
    fn parse_const_i32_module() {
        let file_path = "test/fixtures/const_i32.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let module = parser.parse(&mut bytes).expect("Failed to parse");

        let type_section_function_types = module.type_section.unwrap().function_types;
        assert_eq!(type_section_function_types[0].params.val_types.len(), 0);
        assert_eq!(type_section_function_types[0].returns.val_types.len(), 1);

        assert_eq!(module.function_section.unwrap().type_indexes.len(), 1);

        let code_section_bodies = module.code_section.unwrap().bodies;
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

        let type_section_function_types = module.type_section.unwrap().function_types;
        assert_eq!(type_section_function_types[0].params.val_types.len(), 0);
        assert_eq!(type_section_function_types[0].returns.val_types.len(), 1);

        assert_eq!(module.function_section.unwrap().type_indexes.len(), 1);

        let code_section_bodies = module.code_section.unwrap().bodies;
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

        let type_section_function_types = module.type_section.unwrap().function_types;
        assert_eq!(type_section_function_types[0].params.val_types.len(), 2);
        assert_eq!(type_section_function_types[0].returns.val_types.len(), 1);

        assert_eq!(module.function_section.unwrap().type_indexes.len(), 1);

        let export_section_exports = module.export_section.unwrap().exports;
        assert_eq!(export_section_exports.len(), 1);
        assert_eq!(export_section_exports[0].name, [97, 100, 100]);
        assert_eq!(export_section_exports[0].export_desc.index, 0);
        assert_eq!(
            export_section_exports[0].export_desc.export_type,
            ExportType::Function
        );

        let code_section_bodies = module.code_section.unwrap().bodies;
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

        let type_section_function_types = module.type_section.unwrap().function_types;
        assert_eq!(type_section_function_types[0].params.val_types.len(), 1);
        assert_eq!(type_section_function_types[0].returns.val_types.len(), 1);

        assert_eq!(module.function_section.unwrap().type_indexes, [0]);

        let export_section_exports = module.export_section.unwrap().exports;
        assert_eq!(export_section_exports.len(), 1);
        assert_eq!(export_section_exports[0].name, [103, 101, 95, 115, 49, 48]);
        assert_eq!(export_section_exports[0].export_desc.index, 0);
        assert_eq!(
            export_section_exports[0].export_desc.export_type,
            ExportType::Function
        );

        let code_section_bodies = module.code_section.unwrap().bodies;
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

        let type_section_function_types = module.type_section.unwrap().function_types;
        assert_eq!(type_section_function_types[0].params.val_types.len(), 0);
        assert_eq!(type_section_function_types[0].returns.val_types.len(), 1);

        assert_eq!(module.function_section.unwrap().type_indexes, [0]);

        let export_section_exports = module.export_section.unwrap().exports;
        assert_eq!(export_section_exports.len(), 1);
        assert_eq!(export_section_exports[0].name, [108, 111, 111, 112]);
        assert_eq!(export_section_exports[0].export_desc.index, 0);
        assert_eq!(
            export_section_exports[0].export_desc.export_type,
            ExportType::Function
        );

        let code_section_bodies = module.code_section.unwrap().bodies;
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

        let type_section_function_types = module.type_section.unwrap().function_types;
        assert_eq!(type_section_function_types.len(), 2);
        assert_eq!(type_section_function_types[0].params.val_types.len(), 2);
        assert_eq!(type_section_function_types[0].returns.val_types.len(), 1);
        assert_eq!(type_section_function_types[1].params.val_types.len(), 1);
        assert_eq!(type_section_function_types[1].returns.val_types.len(), 1);

        assert_eq!(module.function_section.unwrap().type_indexes, [0, 1]);

        let export_section_exports = module.export_section.unwrap().exports;
        assert_eq!(export_section_exports.len(), 1);
        assert_eq!(
            export_section_exports[0].name,
            [105, 110, 99, 114, 101, 109, 101, 110, 116]
        );
        assert_eq!(export_section_exports[0].export_desc.index, 1);
        assert_eq!(
            export_section_exports[0].export_desc.export_type,
            ExportType::Function
        );

        let code_section_bodies = module.code_section.unwrap().bodies;
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
        let file_path = "test/fixtures/increment.wasm";
        let mut bytes = std::fs::read(file_path).expect("file not found");
        let parser = parser::Parser::new().unwrap();
        let mut module = parser.parse(&mut bytes).expect("Failed to parse");
        module.emit();

        assert_eq!(module.buffer.bytes, [0, 97, 115, 109, 1, 0, 0, 0,]);
    }
}
