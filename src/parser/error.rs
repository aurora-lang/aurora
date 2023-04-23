#[derive(Debug)]
pub enum ErrorKind {
    ExpressionError,
}

#[derive(Debug)]
pub struct Error {
    pub code: String,
    pub kind: ErrorKind,
    pub message: String
}
