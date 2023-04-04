#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Identifier(Vec<char>),
    Plus(char),
    Minus(char),
    Multiplication(char),
    Division(char),
    Assign(char),
    Bang(char),
    Lt(char),
    Gt(char),
    Semicolon(char),
    Colon(char),
    LParen(char),
    RParen(char),
    Comma(char),
    LBrace(char),
    RBrace(char),
    Int(Vec<char>),
    String(Vec<char>),
    Arrow(String),
    // KEYWORDS
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    End,
    Unkown,
    Whitespace,
    EOF,
}

pub fn get_keyword_token(ident: &Vec<char>) -> Result<Token, String> {
    let identifier: String = ident.into_iter().collect();
    match &identifier[..] {
        "fn" => Ok(Token::Function),
        "let" => Ok(Token::Let),
        "true" => Ok(Token::True),
        "false" => Ok(Token::False),
        "if" => Ok(Token::If),
        "else" => Ok(Token::Else),
        "end" => Ok(Token::End),
        "return" => Ok(Token::Return),
        _ => Err(String::from("Not a keyword")),
    }
}
