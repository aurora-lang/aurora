#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Identifier { val: Vec<char> },
    Plus { val: char },
    Minus { val: char },
    Multiplication { val: char },
    Division { val: char },
    Assign { val: char },
    Bang { val: char },
    Lt { val: char },
    Gt { val: char },
    Semicolon { val: char },
    Colon { val: char },
    LParen { val: char },
    RParen { val: char },
    Comma { val: char },
    LBrace { val: char },
    RBrace { val: char },
    Int { val: Vec<char> },
    Float { val: Vec<char> },
    String { val: Vec<char> },
    Arrow { val: String },
    // KEYWORDS
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    End,
    Module,
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
        "module" => Ok(Token::Module),
        _ => Err(String::from("Not a keyword")),
    }
}
