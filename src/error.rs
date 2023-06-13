use crate::lexer::Lexer;

#[derive(Debug)]
pub enum CompilerErrorKind {
    // ParserError,
    CommandLineError,
}

#[derive(Debug)]
pub struct CompilerError {
    pub code: usize,
    pub kind: CompilerErrorKind,
    pub message: String,
}

pub fn print_error(error: &str, lexer: &Lexer) -> String {
    format!("{} {}:{}", error, lexer.line, lexer.read_position)
}