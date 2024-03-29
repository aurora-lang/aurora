use serde::{Serialize, Deserialize};

use super::{expressions::Expression, Type, FuncParam};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statements {
    VariableDeclaration {
        name: String,
        value: Expression,
        r#type: Type
    },
    ModuleDeclaration {
        name: String
    },
    FunctionDeclaration {
        name: String,
        params: Vec<FuncParam>,
        body: Vec<Statements>,
        return_type: Type
    },
    ExpressionStatement {
        expr: Expression
    }
}