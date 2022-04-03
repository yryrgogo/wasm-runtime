use crate::module::{function::Function, number::Number};

#[derive(Debug)]
pub struct Frame {
    pub function: Function,
    local_vars: Vec<Number>,
}

impl Default for Frame {
    fn default() -> Frame {
        Frame::new(Function::default(), vec![])
    }
}

impl Frame {
    pub fn new(function: Function, mut local_vars: Vec<Number>) -> Frame {
        let mut args = function.create_local_variables();
        args.append(&mut local_vars);
        Frame {
            local_vars: args,
            function,
        }
    }

    pub fn reference_local_var(&self, local_idx: usize) -> &Number {
        &self.local_vars[local_idx]
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
