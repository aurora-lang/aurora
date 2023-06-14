use ast::{statements::Statements, Program};
use error::{CompilerError, CompilerErrorKind};
use inkwell::{context::Context, execution_engine::JitFunction, OptimizationLevel};
use lexer::Lexer;
use parser::parser::Parser;
use std::{
    env::{self},
    fs,
};

use crate::codegen::CodeGen;

mod ast;
mod codegen;
mod error;
mod lexer;
mod parser;
mod utils;

fn compile(input: String, output: String) {
    let code = fs::read_to_string(input).unwrap();
    let mut m_name: String = String::new();
    let mut _lexer = Lexer::new(code.chars().collect());
    let mut _parser = Parser::new(_lexer.clone());
    let mut _ast = _parser.parse();
    let mut program = Program::compile((_ast, _parser.exports));
    let module_statement = program.statements.remove(0);

    if matches!(module_statement, Statements::ModuleDeclaration { .. }) {
        match module_statement {
            Statements::ModuleDeclaration { name } => m_name = name,
            _ => panic!(""),
        }
    }

    let context = Context::create();
    let module = context.create_module(&m_name);
    let code_gen = CodeGen::new();

    for statement in &program.statements {
        match statement {
            // Statements::VariableDeclaration { name, value, r#type } => todo!(),
            // Statements::ModuleDeclaration { name } => todo!(),
            Statements::FunctionDeclaration { .. } => code_gen
                .compile_function_statement(statement.clone(), &module)
                .unwrap(),
            _ => todo!(), // Statements::ExpressionStatement { expr } => todo!(),
        }
    }

    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
    unsafe {
        type Addition = unsafe extern "C" fn(i32, i32) -> i32;
        let add: JitFunction<Addition> = execution_engine.get_function("add").unwrap();
        let x = 1;
        let y = 2;
        println!("{}", add.call(x, y))
    }

    let contents = serde_json::to_string_pretty(&program).unwrap();
    fs::write(output, contents).unwrap();
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
