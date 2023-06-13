use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    StringLiteral { val: String },
    Identifier { val: String },
    IntLiteral { val: i64 },
    BooleanLiteral { val: bool },
    FloatLiteral { val: f64 },
    FunctionCall { name: String, params: Vec<Expression> },
}
