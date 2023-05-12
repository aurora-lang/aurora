use super::tokens;

#[derive(Debug, Clone)]
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

    #[cfg(feature = "debug")]
    pub fn lex(&mut self) -> Vec<tokens::Token> {
        let mut tokens: Vec<tokens::Token> = vec![];
        self.read_char();
        loop {
            let token = self.next_token();
            if token == tokens::Token::EOF {
                break;
            } else {
                if token != tokens::Token::Whitespace && token != tokens::Token::Unkown {
                    tokens.push(token);
                }
            }
        }
        tokens
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
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
    pub fn token_match(&mut self) -> tokens::Token {
        let read_identifier = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && l.ch.is_alphanumeric() {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };
        let read_number = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;

            while l.position < l.input.len() {

                if l.ch.is_numeric() || l.ch == '.' {
                    l.read_char();
                } else {
                    break;
                }
            }

            l.input[position..l.position].to_vec()
        };
        let tok: tokens::Token;
        self.skip_whitespace();
        match self.ch {
            '=' => {
                tok = tokens::Token::Assign { val: self.ch };
            }
            '+' => {
                tok = tokens::Token::Plus { val: self.ch };
            }
            '-' => {
                let cur_ch = self.ch;
                self.read_char();
                if self.ch == '>' {
                    tok = tokens::Token::Arrow {
                        val: [cur_ch, self.ch].into_iter().collect(),
                    }
                } else {
                    tok = tokens::Token::Minus { val: self.ch };
                }
            }
            '!' => {
                tok = tokens::Token::Bang { val: self.ch };
            }
            '/' => {
                tok = tokens::Token::Division { val: self.ch };
            }
            '*' => {
                tok = tokens::Token::Multiplication { val: self.ch };
            }
            '<' => {
                tok = tokens::Token::Lt { val: self.ch };
            }
            '>' => {
                tok = tokens::Token::Gt { val: self.ch };
            }
            ';' => {
                tok = tokens::Token::Semicolon { val: self.ch };
            }
            ':' => {
                tok = tokens::Token::Colon { val: self.ch };
            }
            '(' => {
                tok = tokens::Token::LParen { val: self.ch };
            }
            ')' => {
                tok = tokens::Token::RParen { val: self.ch };
            }
            ',' => {
                tok = tokens::Token::Comma { val: self.ch };
            }
            '{' => {
                tok = tokens::Token::LBrace { val: self.ch };
            }
            '}' => {
                tok = tokens::Token::RBrace { val: self.ch };
            }
            '\0' => {
                tok = tokens::Token::EOF;
            }
            _ if self.ch == '"' => {
                let mut stri: Vec<char> = vec![];
                self.read_char();
                while self.ch != '"' {
                    if self.ch == '\\' {
                        self.read_char();
                        stri.push(self.ch);
                    }
                    stri.push(self.ch);
                    self.read_char();
                }
                self.read_char();
                return tokens::Token::String { val: stri };
            }
            _ if self.ch.is_alphabetic() => {
                let ident: Vec<char> = read_identifier(self);
                match tokens::get_keyword_token(&ident) {
                    Ok(keywork_token) => {
                        return keywork_token;
                    }
                    Err(_err) => {
                        return tokens::Token::Identifier { val: ident };
                    }
                }
            }
            _ if self.ch.is_numeric() => {
                let ident: Vec<char> = read_number(self);
                if ident.contains(&'.') {
                    return tokens::Token::Float { val: ident };
                }

                return tokens::Token::Int { val: ident };
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

    pub fn next_token(&mut self) -> tokens::Token {
        let mut t = self.token_match();
        if matches!(t, tokens::Token::Whitespace) {
            t = self.token_match()
        }
        t
    }
    pub fn peak_next_token(&mut self) -> tokens::Token {
        let old_ch = self.ch;
        let old_postion = self.position;
        let old_read_postion = self.position;

        let token = self.token_match();

        self.ch = old_ch;
        self.position = old_postion;
        self.read_position = old_read_postion;

        token
    }
}
