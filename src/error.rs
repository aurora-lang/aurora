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
