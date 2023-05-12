use crate::{
    ast::{expressions::Expression, statements::Statements, FuncParam, Type},
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

        while let Ok(statement) = self.parse_statement() {
            program.push(statement)
        }

        program
    }

    pub fn parse_block(&mut self) -> Vec<Statements> {
        let mut block: Vec<Statements> = vec![];

        loop {
            let mut curr_token = self.lexer.next_token();

            if matches!(curr_token, Token::Whitespace) {
                curr_token = self.lexer.next_token();
            }

            if matches!(curr_token, Token::End) {
                break;
            }

            if let Ok(statement) = self.parse_statement() {
                block.push(statement);
            } else {
                panic!("")
            }
        }

        self.lexer.next_token();

        block
    }

    fn parse_statement(&mut self) -> Result<Statements, String> {
        let mut curr_token = self.lexer.next_token();

        if matches!(curr_token, Token::Whitespace) {
            curr_token = self.lexer.next_token();
        }

        if matches!(curr_token, Token::EOF) {
            return Err("Reached end of file".to_string());
        }

        // Variable Declaration
        if matches!(curr_token, Token::Let) {
            let id = if matches!(self.lexer.peak_next_token(), Token::Identifier { .. }) {
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
            };

            let expr = self.parse_expr().unwrap();
            return Ok(Statements::VariableDeclaration {
                name: id,
                value: expr,
                r#type: _type,
            });
        }

        // Module Declaration
        if matches!(curr_token, Token::Module) {
            let id = if matches!(self.lexer.peak_next_token(), Token::Identifier { .. }) {
                match self.lexer.next_token() {
                    Token::Identifier { val } => String::from_iter(val),
                    _ => panic!("unexpected identifier"),
                }
            } else {
                panic!("Expected module name")
            };

            return Ok(Statements::ModuleDeclaration { name: id });
        }

        // Function Declaration
        if matches!(curr_token, Token::Function) {
            let id = if matches!(self.lexer.peak_next_token(), Token::Identifier { .. }) {
                match self.lexer.next_token() {
                    Token::Identifier { val } => String::from_iter(val),
                    _ => panic!("unexpected identifier"),
                }
            } else {
                panic!("Expected an identifier")
            };

            if !matches!(self.lexer.next_token(), Token::LParen { .. }) {
                panic!("expected '('");
            }

            let mut parameters: Vec<FuncParam> = vec![];

            loop {
                let mut curr_tk = self.lexer.next_token();

                if matches!(curr_tk, Token::RParen { .. }) {
                    break;
                }

                if matches!(curr_tk, Token::Comma { .. }) {
                    curr_tk = self.lexer.next_token();
                }

                let id = match curr_tk {
                    Token::Identifier { val } => String::from_iter(val),
                    _ => panic!("unexpected identifier"),
                };

                if !matches!(self.lexer.peak_next_token(), Token::Colon { .. }) {
                    panic!("Expected ':'")
                } else {
                    self.lexer.next_token();
                    self.lexer.next_token();
                }

                let _type = if matches!(self.lexer.peak_next_token(), Token::Identifier { .. }) {
                    let raw_type = match self.lexer.next_token() {
                        Token::Identifier { val } => String::from_iter(val),
                        x => panic!("{:?}", x),
                    };
                    Type::parse_type(raw_type)
                } else {
                    panic!("expected type")
                };

                parameters.push(FuncParam {
                    name: id,
                    r#type: _type,
                })
            }

            let body = self.parse_block();

            return Ok(Statements::FunctionDeclaration {
                name: id,
                params: parameters,
                body,
                return_type: Type::Int32,
            });
        }

        Err("Unknown keyword found".to_string())
    }

    fn parse_expr(&mut self) -> Result<Expression, error::Error> {
        match self.lexer.next_token() {
            Token::String { val } => Ok(Expression::StringLiteral {
                val: String::from_iter(val),
            }),
            Token::Int { val } => {
                let val = String::from_iter(val);
                return Ok(Expression::IntLiteral {
                    val: val.trim().parse().unwrap(),
                });
            }
            Token::Float { val } => {
                let val = String::from_iter(val);
                Ok(Expression::FloatLiteral {
                    val: val.trim().parse().unwrap(),
                })
            }
            Token::False => Ok(Expression::BooleanLiteral { val: false }),
            Token::True => Ok(Expression::BooleanLiteral { val: true }),
            Token::Identifier { val } => {
                let val = String::from_iter(val);

                if matches!(self.lexer.peak_next_token(), Token::LParen { .. }) {
                    self.lexer.next_token();
                    self.lexer.next_token();

                    loop {
                        if matches!(self.lexer.peak_next_token(), Token::EOF) {
                            return Err(error::Error {
                                code: "AUR3000".to_string(),
                                kind: error::ErrorKind::ExpressionError,
                                message: "unexpected end of file".to_string(),
                            });
                        }

                        if matches!(self.lexer.peak_next_token(), Token::RParen { .. }) {
                            self.lexer.next_token();
                            break;
                        }

                        if matches!(self.lexer.peak_next_token(), Token::Comma { .. }) {
                            self.lexer.next_token();
                        }

                        println!("{:?}", self.lexer.next_token())
                    }

                    return Ok(Expression::FunctionCall {
                        name: val,
                        params: vec![],
                    });
                }

                Ok(Expression::Identifier { val })
            }
            _ => unreachable!(),
        }
    }
}
