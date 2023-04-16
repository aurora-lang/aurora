use super::{expressions::Expression, types::Type};


#[derive(Debug, Clone)]
pub enum Statements {
    VariableDeclation {
        name: String,
        value: Expression,
        r#type: Type
    }
}