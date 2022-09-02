use std::env;

mod instruction;
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
    let module = parser.parse(&mut bytes).expect("Failed to parse");
    println!("Successfully parse module\n{:#?}", module);
}

#[cfg(test)]
mod parser_tests {
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
}
