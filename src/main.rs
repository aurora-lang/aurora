use lexer::Lexer;
use parser::Parser;
use std::{
    env::{args as env_args, Args},
    fs,
};

mod lexer;
mod parser;

fn uncons(xs: &mut Vec<String>) -> (String, Vec<String>) {
    xs.remove(0);
    (String::from(&xs[0]), xs.to_vec())
}

fn use_args(args: Args) -> Vec<String> {
    let mut arg_vec: Vec<String> = vec![];
    for arg in args {
        arg_vec.push(arg)
    }
    arg_vec
}

fn main() {
    let mut args: Vec<String> = use_args(env_args());
    let (file, _) = uncons(&mut args);

    let code = fs::read_to_string(file).unwrap();

    let mut _lexer = Lexer::new(code.chars().collect());
    // let tokens = _lexer.lex();
    // println!("Tokens:");
    // for token in &tokens {
    //     println!("{:?}", token)
    // }
    let mut _parser = Parser::new(_lexer);
    let exprs = match _parser.parse() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("[PARSER] Error: {}", e.message);
            return;
        },
    };

    println!("Expressions:");
    for expr in exprs {
        println!("{:?}", expr)
    }
}
