use crate::lexer::{tokens::Token, Lexer};

use super::error::{ParserError, ParserErrorKind};
use super::expressions::{CodeBlock, Expression};
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut expr: Vec<Expression> = vec![];

        while let token = self.lexer.next_token() {
            match token {
                Token::Let => {
                    let id = self.lexer.next_token();
                    if id != Token::Identifier(vec![]) {}

                    let id = match id {
                        Token::Identifier(c) => Expression::StringLiteral(String::from_iter(c)),
                        _ => todo!(),
                    };

                    if !matches!(self.lexer.next_token(), Token::Assign(..)) {
                        return Err(ParserError {
                            code: 0,
                            kind: ParserErrorKind::ExpectedToken,
                            message: format!("Expected token '=' @ {}", self.lexer.position),
                        });
                    }

                    let value = self.parse_expr();

                    if value == Expression::EOF {
                        return Err(ParserError {
                            code: 1,
                            kind: ParserErrorKind::ExpectedToken,
                            message: format!("Expected expression but found End Of File"),
                        });
                    }

                    expr.push(Expression::DeclaceVariable(Box::new(id), Box::new(value)));
                }
                Token::Identifier(c) => {
                    let id = String::from_iter(c);

                    if self.lexer.next_token() == Token::LParen('(') {
                        let mut func_args: Vec<Expression> = vec![];
                        while let tk = self.parse_expr() {
                            if tk == Expression::Bracket(')') || tk == Expression::EOF {
                                break;
                            }
                            if tk != Expression::Sep(vec![',']) {
                                func_args.push(tk.clone());
                            }
                        }
                        expr.push(Expression::Call(id, func_args))
                    }
                }
                Token::Plus(_) => todo!(),
                Token::Minus(_) => todo!(),
                Token::Multiplication(_) => todo!(),
                Token::Division(_) => todo!(),
                Token::Assign(_) => todo!(),
                Token::Bang(_) => todo!(),
                Token::Lt(_) => todo!(),
                Token::Gt(_) => todo!(),
                Token::Semicolon(_) => todo!(),
                Token::Colon(_) => todo!(),
                Token::LParen(_) => todo!(),
                Token::RParen(_) => todo!(),
                Token::Comma(_) => todo!(),
                Token::LBrace(_) => todo!(),
                Token::RBrace(_) => todo!(),
                Token::Int(_) => todo!(),
                Token::String(_) => todo!(),
                Token::Arrow(s) => {}
                Token::Function => {
                    let func_name = self.parse_expr();
                    if let Expression::Identifier(id) = func_name {
                        let mut func_args: Vec<String> = vec![];
                        while let tk = self.parse_expr() {
                            if tk == Expression::Bracket(')') || tk == Expression::EOF {
                                break;
                            }
                            if tk != Expression::Sep(vec![',']) {
                                if let Expression::Identifier(arg) = tk {
                                    func_args.push(arg);
                                }
                            }
                        }
                        self.parse_expr();
                        let return_type = self.parse_expr();
                        let mut code_block: CodeBlock = vec![];
                        while let tk = self.parse_expr() {
                            if tk == Expression::FunctionLiteralEnd || tk == Expression::EOF {
                                break;
                            }

                            if tk != Expression::Sep(vec!['\n', ' ', '\t', '\r']) {
                                code_block.push(tk.clone());
                            }
                        }

                        expr.push(Expression::FunctionLiteral(
                            id,
                            func_args,
                            Box::new(return_type),
                            code_block,
                        ))
                    } else {
                        return Err(ParserError {
                            code: 2,
                            kind: ParserErrorKind::ExpectedToken,
                            message: format!(
                                "Exected {:?} but could found {:?}",
                                Expression::Identifier(String::default()),
                                func_name
                            ),
                        });
                    }
                }
                Token::True => todo!(),
                Token::False => todo!(),
                Token::If => todo!(),
                Token::Else => todo!(),
                Token::Return => todo!(),
                Token::End => {}
                Token::Unkown => todo!(),
                Token::Whitespace => {}
                Token::EOF => {
                    break;
                }
            }
        }

        Ok(expr)
    }

    pub fn parse_expr(&mut self) -> Expression {
        match self.lexer.next_token() {
            Token::Identifier(c) => Expression::Identifier(String::from_iter(c)),
            Token::Plus(_) => todo!(),
            Token::Minus(_) => todo!(),
            Token::Multiplication(_) => todo!(),
            Token::Division(_) => todo!(),
            Token::Assign(_) => Expression::Sep(vec!['=']),
            Token::Bang(_) => todo!(),
            Token::Lt(_) => todo!(),
            Token::Gt(_) => todo!(),
            Token::Semicolon(_) => todo!(),
            Token::Colon(_) => todo!(),
            Token::LParen(_) => Expression::Bracket('('),
            Token::RParen(_) => Expression::Bracket(')'),
            Token::Comma(_) => Expression::Sep(vec![',']),
            Token::LBrace(_) => todo!(),
            Token::RBrace(_) => todo!(),
            Token::Int(i) => Expression::IntegerLiteral(String::from_iter(i).parse().unwrap()),
            Token::String(c) => Expression::StringLiteral(String::from_iter(c)),
            Token::Arrow(_) => Expression::Sep(vec![]),
            Token::Function => todo!(),
            Token::Let => Expression::Keyword("let".to_string()),
            Token::True => Expression::Boolean(true),
            Token::False => Expression::Boolean(false),
            Token::If => todo!(),
            Token::Else => todo!(),
            Token::Return => todo!(),
            Token::End => Expression::FunctionLiteralEnd,
            Token::Unkown => todo!(),
            Token::Whitespace => Expression::Sep(vec!['\n', ' ', '\t', '\r']),
            Token::EOF => Expression::EOF,
        }
    }
}
