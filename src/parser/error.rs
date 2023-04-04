#[derive(Debug, Default)]
pub enum ParserErrorKind {
    
    #[default] GenericError,
    ExpectedToken
}


#[derive(Debug, Default)]
pub struct ParserError {
    pub code: usize,
    pub kind: ParserErrorKind,
    pub message: String,
}