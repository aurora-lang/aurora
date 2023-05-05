use ast::statements::Statements;
use error::{CompilerError, CompilerErrorKind};
use lexer::Lexer;
use parser::parser::Parser;
use std::{
    env::{self},
    fs,
};

mod ast;
mod error;
mod lexer;
mod parser;

fn compile(input: String, output: String) {
    let code = fs::read_to_string(input).unwrap();
    let mut _lexer = Lexer::new(code.chars().collect());
    let mut _parser = Parser::new(_lexer);

    for statement in _parser.parse() {
        if matches!(statement, Statements::ModuleDeclation { .. }) {
            match statement {
                Statements::ModuleDeclation { name } => println!("{name}"),
                _ => panic!("")
            }
        }
    }
    

    println!("output: {output}")
}

fn main() -> Result<(), CompilerError> {
    let args = {
        let args = env::args().into_iter();
        let mut arg_str: Vec<String> = vec![];
        for e in args {
            arg_str.push(e)
        }
        arg_str
    };

    let mut output: String = "a.out".to_string();

    if args.len() < 2 {
        return Err(CompilerError {
            code: 01,
            kind: CompilerErrorKind::CommandLineError,
            message: "Not enough arguments provided".to_string(),
        });
    }

    let mut pos: usize = 0;
    let input = (&args.clone()[pos + 1]).to_owned();
    for arg in &args {
        if arg == "--output" || arg == "-o" {
            let o = &args.clone()[pos + 1];
            output = o.to_string();
        }

        pos = pos + 1
    }

    compile(input, output);

    Ok(())
}
