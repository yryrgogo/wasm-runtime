use crate::module::{function::Function, number::Number};

#[derive(Debug, Clone)]
pub struct Frame {
    pub function: Function,
    pub local_vars: Vec<Number>,
}

impl Default for Frame {
    fn default() -> Frame {
        Frame::new(Function::default(), vec![])
    }
}

impl Frame {
    pub fn new(function: Function, mut local_vars: Vec<Number>) -> Frame {
        local_vars.append(&mut function.create_local_variables());
        Frame {
            local_vars: local_vars,
            function,
        }
    }

    pub fn reference_local_var(&self, local_idx: usize) -> Number {
        self.local_vars.get(local_idx).unwrap().clone()
    }

    pub fn inspect(&self) -> String {
        format!(
            "#<Frame local={}>",
            self.local_vars
                .iter()
                .map(|x| format!("{}", x.inspect()))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
