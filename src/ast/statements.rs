use super::{expressions::Expression, Type, FuncParam};


#[derive(Debug, Clone)]
pub enum Statements {
    VariableDeclation {
        name: String,
        value: Expression,
        r#type: Type
    },
    FunctionDeclation {
        name: String,
        params: Vec<FuncParam>,
        return_type: Type
    }
}