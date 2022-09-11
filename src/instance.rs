use std::collections::HashMap;

use crate::{
    module::ModuleNode,
    node::{CodeNode, ExportTypeNode, FunctionTypeNode, InstructionNode},
    types::ValueType,
};

#[derive(Debug, Clone)]
pub struct Instance {
    pub exportMap: HashMap<String, Export>,
    pub functions: Vec<Function>,
}

impl Instance {
    pub fn new(module: &ModuleNode) -> Self {
        let functions = Instance::instantiate_functions(module);
        let exportMap = Instance::instantiate_exports(module);

        Instance {
            exportMap,
            functions,
        }
    }

    pub fn instantiate_functions(module: &ModuleNode) -> Vec<Function> {
        let mut functions: Vec<Function> = vec![];
        let function_types = module
            .type_section()
            .unwrap_or_else(|| {
                panic!("Module does not have a type section");
            })
            .function_types
            .clone();
        let code_section_bodies = module
            .code_section()
            .unwrap_or_else(|| {
                panic!("Module does not have a code section");
            })
            .bodies
            .clone();

        for type_index in module
            .function_section()
            .unwrap_or_else(|| {
                panic!("Module does not have a function section");
            })
            .clone()
            .type_indexes
        {
            functions.push(Function::new(
                &function_types[type_index as usize],
                &code_section_bodies[type_index as usize],
            ));
        }
        functions
    }

    pub fn instantiate_exports(module: &ModuleNode) -> HashMap<String, Export> {
        let mut exports: HashMap<String, Export> = HashMap::new();
        for export in module
            .export_section()
            .unwrap_or_else(|| panic!(""))
            .exports
            .iter()
        {
            match &export.export_desc.export_type {
                ExportTypeNode::Function => {
                    exports
                        .entry(export.name.clone())
                        .and_modify(|_| panic!(""))
                        .or_insert(Export::Function {
                            index: export.export_desc.index as usize,
                            name: export.name.clone(),
                        });
                }
                ExportTypeNode::Table => {}
                ExportTypeNode::Memory => {}
                ExportTypeNode::Global => {}
            }
        }
        exports
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub locals: Vec<Local>,
    pub instructions: Vec<InstructionNode>,
    pub params: Vec<ValueType>,
    pub returns: Vec<ValueType>,
}

impl Function {
    fn new(function_type: &FunctionTypeNode, code: &CodeNode) -> Self {
        let locals = Vec::from_iter(code.locals.iter().map(|local| Local {
            name: None,
            val_type: local.val_type,
        }));
        let f = function_type.clone();
        Function {
            locals,
            instructions: code.expr.instructions.clone(),
            params: f.params.val_types,
            returns: f.returns.val_types,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Local {
    name: Option<String>,
    val_type: ValueType,
}

#[derive(Debug, Clone)]
pub enum Export {
    Function { name: String, index: usize },
    Table { name: String, index: usize },
    Memory { name: String, index: usize },
    Global { name: String, index: usize },
}
