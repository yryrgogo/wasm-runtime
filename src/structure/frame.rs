use crate::module::number::Number;

struct Frame {
    local_vars: Vec<Number>,
}

impl Frame {
    pub fn new(local_vars: Vec<Number>) -> Frame {
        Frame {
            local_vars: local_vars,
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
