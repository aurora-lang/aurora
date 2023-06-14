use crate::{
    ast::{expressions::Expression, statements::Statements, Export, ExportType, FuncParam, Type},
    error::print_error,
    lexer::{tokens::Token, Lexer},
};

use super::error;
pub struct Parser {
    lexer: Lexer,
    pub exports: Vec<Export>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            exports: vec![],
        }
    }

    pub fn parse(&mut self) -> Vec<Statements> {
        let mut program: Vec<Statements> = vec![];
        loop {
            let statement = match self.parse_statement() {
                Ok(statement) => statement,
                Err(_) => break,
            };

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

            if matches!(self.lexer.peak_next_token(), Token::End) {
                self.lexer.next_token();
                break;
            }

            match self.parse_statement() {
                Ok(statement) => block.push(statement),
                Err(e) => {
                    println!("{}", print_error(&e, &self.lexer));
                    return block;
                },
            }
        }

        block
    }

    pub fn parse_statement(&mut self) -> Result<Statements, String> {
        let mut curr_token = self.lexer.next_token();
        let mut public = false;

        if matches!(curr_token, Token::Whitespace) {
            curr_token = self.lexer.next_token();
        }

        if matches!(curr_token, Token::EOF) {
            return Err("Reached end of file".to_string());
        }

        if matches!(curr_token, Token::Public) {
            public = true;
            curr_token = self.lexer.next_token();
        }

        if matches!(curr_token, Token::Let) {
            let id = if matches!(self.lexer.peak_next_token(), Token::Identifier { .. }) {
                match self.lexer.next_token() {
                    Token::Identifier { val } => String::from_iter(val),
                    _ => panic!("unexpected identifier"),
                }
            } else {
                panic!("{}", print_error("Expected an identifier", &self.lexer))
            };

            if !matches!(self.lexer.peak_next_token(), Token::Colon { .. }) {
                panic!("{}", print_error("Expected ':'", &self.lexer))
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
                panic!("{}", print_error("expected type", &self.lexer))
            };

            if !matches!(self.lexer.peak_next_token(), Token::Assign { .. }) {
                panic!(
                    "{}",
                    print_error("Expected assignment operoator '='", &self.lexer)
                )
            } else {
                self.lexer.next_token();
            };

            let expr = self.parse_expr().unwrap();
            return Ok(Statements::VariableDeclaration {
                name: id,
                value: expr,
                r#type: _type,
            });
        } else
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
        } else
        // Function Declaration
        if matches!(curr_token, Token::Function) {
            let id = if matches!(self.lexer.peak_next_token(), Token::Identifier { .. }) {
                match self.lexer.next_token() {
                    Token::Identifier { val } => String::from_iter(val),
                    _ => panic!("unexpected identifier"),
                }
            } else {
                panic!("{}", print_error("Expected an identifier", &self.lexer))
            };

            if !matches!(self.lexer.next_token(), Token::LParen { .. }) {
                panic!("{}", print_error("expected '('", &self.lexer));
            }

            let mut parameters: Vec<FuncParam> = vec![];

            // handle function patams
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

            if matches!(self.lexer.peak_next_token(), Token::Arrow { .. }) {
                self.lexer.next_token();
            }

            let ret_type = if matches!(self.lexer.peak_next_token(), Token::Identifier { .. }) {
                let raw_type = match self.lexer.next_token() {
                    Token::Identifier { val } => String::from_iter(val),
                    x => panic!("{:?}", x),
                };
                Type::parse_type(raw_type)
            } else {
                Type::Void
            };

            let body = self.parse_block();

            // if !matches!(self.lexer.next_token(), Token::End) {
            //     return Err(print_error("error", &self.lexer));
            // }

            self.lexer.next_token();

            let func = Statements::FunctionDeclaration {
                name: id,
                params: parameters,
                body,
                return_type: ret_type,
            };

            if public {
                self.exports.push(Export {
                    r#type: ExportType::Function,
                    statement: func.clone(),
                })
            }

            return Ok(func);
        } else if let Ok(expr) = self.parse_expr() {
            Ok(Statements::ExpressionStatement { expr })
        } else {
            return Err(format!("Unknown keyword found: {:#?}", curr_token));
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expression, error::Error> {
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

                    let mut params: Vec<Expression> = vec![];

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
                            self.lexer.next_token();
                        }

                        params.push(self.parse_expr().unwrap());
                    }

                    return Ok(Expression::FunctionCall { name: val, params });
                }

                Ok(Expression::Identifier { val })
            }

            x => Err(error::Error {
                code: "AUR7000".to_string(),
                kind: error::ErrorKind::ExpressionError,
                message: print_error(&format!("can not parse this token {:#?}", x), &self.lexer),
            }),
        }
    }
}
