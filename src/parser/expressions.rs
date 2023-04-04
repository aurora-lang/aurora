
pub type CodeBlock = Vec<Expression>;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    StringLiteral(String),
    Sep(Vec<char>),
    Bracket(char),
    Boolean(bool),
    // Array(Vec<Expression>),
    // Hash(HashLiteral)
    // Index(Box<Expression>, Box<Expression>),
    DeclaceVariable(Box<Expression>, Box<Expression>),
    Keyword(String),
    // Prefix(Prefix, Box<Expression>),
    // Infix(Infix, Box<Expression>, Box<Expression>),
    // If(Box<Expression>, BlockStatement, Option<BlockStatement>),
    FunctionLiteral(String, Vec<String>, Box<Expression>, CodeBlock),
    FunctionLiteralEnd,
    Call(String, Vec<Expression>),
    EOF
}