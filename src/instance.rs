use std::collections::HashMap;

use crate::{
    module::ModuleNode,
    node::{CodeNode, ExportTypeNode, FunctionNode, FunctionTypeNode, InstructionNode},
    types::ValueType,
};

#[derive(Debug, Clone)]
pub struct Instance {
    pub exportMap: HashMap<String, Export>,
    pub functions: Vec<FunctionInstance>,
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

    pub fn instantiate_functions(module: &ModuleNode) -> Vec<FunctionInstance> {
        let mut functions: Vec<FunctionInstance> = vec![];
        let function_types = module
            .type_section()
            .unwrap_or_else(|| {
                panic!("Module does not have a type section");
            })
            .function_types
            .clone();
        if let Some(funcs) = module.funcs() {
            for type_index in module
                .function_section()
                .unwrap_or_else(|| {
                    panic!("Module does not have a function section");
                })
                .clone()
                .type_indexes
            {
                functions.push(FunctionInstance::new(
                    &function_types[type_index as usize],
                    funcs[type_index as usize].clone(),
                ));
            }
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
pub struct FunctionInstance {
    pub function_type: FunctionTypeNode,
    pub code: FunctionNode,
}

impl FunctionInstance {
    fn new(function_type: &FunctionTypeNode, code: FunctionNode) -> Self {
        FunctionInstance {
            function_type: function_type.clone(),
            code,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Export {
    Function { name: String, index: usize },
    Table { name: String, index: usize },
    Memory { name: String, index: usize },
    Global { name: String, index: usize },
}
