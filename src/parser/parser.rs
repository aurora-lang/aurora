use crate::{
    ast::{expressions::Expression, statements::Statements, Type},
    lexer::{tokens::Token, Lexer},
};

use super::error;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Vec<Statements> {
        let mut program: Vec<Statements> = vec![];

        loop {
            let curr_token = self.lexer.next_token();
            if matches!(curr_token, Token::EOF) {
                break;
            }

            if matches!(curr_token, Token::Let) {
                let id = if matches!(self.lexer.peak_next_token(), Token::Identifier { .. }) {
                    self.lexer.next_token();
                    match self.lexer.next_token() {
                        Token::Identifier { val } => String::from_iter(val),
                        _ => panic!("unexpected identifier"),
                    }
                } else {
                    panic!("Expected an identifier")
                };

                if !matches!(self.lexer.peak_next_token(), Token::Colon { .. }) {
                    panic!("Expected ':'")
                } else {
                    self.lexer.next_token();
                    self.lexer.next_token();
                }

                let _type = if matches!(self.lexer.peak_next_token(), Token::Identifier { .. }) {
                    self.lexer.next_token();
                    let raw_type = match self.lexer.next_token() {
                        Token::Identifier { val } => String::from_iter(val),
                        x => panic!("{:?}", x),
                    };
                    Type::parse_type(raw_type)
                } else {
                    panic!("expected type")
                };

                if !matches!(self.lexer.peak_next_token(), Token::Assign { .. }) {
                    panic!("Expected assignment operoator '='")
                } else {
                    self.lexer.next_token();
                    self.lexer.next_token()
                };

                let expr = self.parse_expr().unwrap();
                program.push(Statements::VariableDeclation {
                    name: id,
                    value: expr,
                    r#type: _type,
                })
            }

            if matches!(curr_token, Token::Module) {
                let id = if matches!(self.lexer.peak_next_token(), Token::Identifier { .. }) {
                    self.lexer.next_token();
                    match self.lexer.next_token() {
                        Token::Identifier { val } => String::from_iter(val),
                        _ => panic!("unexpected identifier"),
                    }
                } else {
                    panic!("Expected module name")
                };

                program.push(Statements::ModuleDeclation { name: id })
            }
        }

        program
    }

    fn parse_expr(&mut self) -> Result<Expression, error::Error> {
        match self.lexer.next_token() {
            Token::String { val } => Ok(Expression::StringLiteral {
                val: String::from_iter(val),
            }),
            Token::Int { val } => Ok(Expression::IntLiteral {
                val: String::from_iter(val).parse().unwrap(),
            }),
            Token::Float { val } => Ok(Expression::FloatLiteral {
                val: String::from_iter(val).parse().unwrap(),
            }),
            Token::False => Ok(Expression::BooleanLiteral { val: false }),
            Token::True => Ok(Expression::BooleanLiteral { val: true }),
            Token::Identifier { val } => {
                let val = String::from_iter(val);

                if matches!(self.lexer.peak_next_token(), Token::LParen { .. }) {
                    self.lexer.next_token();
                    let mut params: Vec<Expression> = vec![];
                    loop {
                        if matches!(self.lexer.next_token(), Token::EOF) {
                            break;
                        }
                        if matches!(self.lexer.peak_next_token(), Token::RParen { .. }) {
                            self.lexer.next_token();
                            break;
                        }
                        params.push(self.parse_expr().unwrap());
                    }

                    return Ok(Expression::FunctionCall { name: val, params });
                }

                Ok(Expression::Identifier { val })
            }
            x => Err(error::Error {
                code: "AUR3000".to_string(),
                kind: error::ErrorKind::ExpressionError,
                message: format!("Unknow implemented token {:?}", x),
            }),
        }
    }
}
