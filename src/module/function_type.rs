use super::number::Number;

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub parameters: Vec<Number>,
    pub results: Vec<Number>,
}
impl Default for FunctionType {
    fn default() -> FunctionType {
        FunctionType {
            parameters: vec![],
            results: vec![],
        }
    }
}
impl FunctionType {
    pub fn inspect(&self) -> String {
        let params = self
            .parameters
            .iter()
            .map(|x| format!("{}", x.inspect()))
            .collect::<Vec<String>>()
            .join(", ");
        let results = self
            .results
            .iter()
            .map(|x| format!("{}", x.inspect()))
            .collect::<Vec<String>>()
            .join(", ");

        format!("({}) => ({})", params, results)
    }
}
