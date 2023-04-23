use std::string;

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

        while let curr_token = self.lexer.next_token() {
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
                    // self.lexer.next_token();
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
                    r#type: _type
                })
            }
        }

        program
    }

    fn parse_expr(&mut self) -> Result<Expression, error::Error> {
        match self.lexer.next_token() {
            Token::String { val } => Ok(Expression::StringLiteral {
                val: String::from_iter(val),
            }),
            Token::False => Ok(Expression::BooleanLiteral { val: false }),
            Token::True => Ok(Expression::BooleanLiteral { val: true }),
            x => panic!("{:?}", x),
        }
    }
}
