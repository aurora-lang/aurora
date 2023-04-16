#[derive(Debug, Clone)]
pub enum Expression {
    StringLiteral { val: String },
    IntLiteral { val: i64 },
    BooleanLiteral { val: bool },
    FloatLiteral { val: f64 },
    FunctionCall { name: String, params: Vec<Expression> }
}
