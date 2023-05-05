use super::{expressions::Expression, Type};


#[derive(Debug, Clone)]
pub enum Statements {
    VariableDeclation {
        name: String,
        value: Expression,
        r#type: Type
    },
    ModuleDeclation {
        name: String
    },
    // FunctionDeclation {
    //     name: String,
    //     params: Vec<FuncParam>,
    //     return_type: Type
    // }
}