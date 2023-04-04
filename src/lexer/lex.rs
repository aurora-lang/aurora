use super::tokens;

pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(source_code: Vec<char>) -> Self {
        Self {
            input: source_code,
            position: 0,
            read_position: 0,
            ch: ' ',
        }
    }

    // pub fn lex(&mut self) -> Vec<tokens::Token> {
    //     let mut tokens: Vec<tokens::Token> = vec![];
    //     self.read_char();
    //     loop {
    //         let token = self.next_token();
    //         if token == tokens::Token::EOF {
    //             break;
    //         } else {
    //             if token != tokens::Token::Whitespace {
    //                 tokens.push(token);
    //             }
    //         }
    //     }
    //     tokens
    // }
    
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn skip_whitespace(&mut self) {
        let ch = self.ch;
        if ch.is_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> tokens::Token {
        let read_identifier = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && l.ch.is_alphanumeric() {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_number = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && l.ch.is_numeric() {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let tok: tokens::Token;
        self.skip_whitespace();
        match self.ch {
            '=' => {
                tok = tokens::Token::Assign(self.ch);
            }
            '+' => {
                tok = tokens::Token::Plus(self.ch);
            }
            '-' => {
                let cur_ch = self.ch;
                self.read_char();
                if self.ch == '>' {
                    tok = tokens::Token::Arrow([cur_ch, self.ch].into_iter().collect())
                } else {
                    tok = tokens::Token::Minus(self.ch);
                }
            }
            '!' => {
                tok = tokens::Token::Bang(self.ch);
            }
            '/' => {
                tok = tokens::Token::Division(self.ch);
            }
            '*' => {
                tok = tokens::Token::Multiplication(self.ch);
            }
            '<' => {
                tok = tokens::Token::Lt(self.ch);
            }
            '>' => {
                tok = tokens::Token::Gt(self.ch);
            }
            ';' => {
                tok = tokens::Token::Semicolon(self.ch);
            }
            ':' => {
                tok = tokens::Token::Colon(self.ch);
            }
            '(' => {
                tok = tokens::Token::LParen(self.ch);
            }
            ')' => {
                tok = tokens::Token::RParen(self.ch);
            }
            ',' => {
                tok = tokens::Token::Comma(self.ch);
            }
            '{' => {
                tok = tokens::Token::LBrace(self.ch);
            }
            '}' => {
                tok = tokens::Token::RBrace(self.ch);
            }
            '0' => {
                tok = tokens::Token::EOF;
            }
            _ if self.ch == '"' => {
                let mut str: Vec<char> = vec![];
                self.read_char();
                while self.ch != '"' {

                    if  self.ch == '\\' {
                        self.read_char();
                        str.push(self.ch);
                    }

                    str.push(self.ch);
                    self.read_char();
                }
                self.read_char();
                return tokens::Token::String(str);
            }
            _ if self.ch.is_alphabetic() => {
                let ident: Vec<char> = read_identifier(self);
                match tokens::get_keyword_token(&ident) {
                    Ok(keywork_token) => {
                        return keywork_token;
                    }
                    Err(_err) => {
                        return tokens::Token::Identifier(ident);
                    }
                }
            }
            _ if self.ch.is_numeric() => {
                let ident: Vec<char> = read_number(self);
                return tokens::Token::Int(ident);
            }
            _ => {
                if self.ch == '\n' || self.ch == '\n' || self.ch == ' ' || self.ch == '\r' {
                    return tokens::Token::Whitespace;
                }
                eprintln!(
                    "[LEXER] Error: Unknown token found @ position {} '{}'",
                    self.position, self.ch
                );
                return tokens::Token::Unkown;
            }
        }
        self.read_char();
        tok
    }
}
