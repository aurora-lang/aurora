use ast::{
    llvm::{LLVMContext, Program},
    statements::Statements,
};
use error::{CompilerError, CompilerErrorKind};
use inkwell::context::Context;
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
    // println!("{:?}", _lexer.lex());
    let mut _parser = Parser::new(_lexer.clone());
    let mut m_name: String = String::new();

    let mut ast = _parser.parse();

    let module_statemet = ast.remove(0);

    if matches!(module_statemet, Statements::ModuleDeclaration { .. }) {
        match module_statemet {
            Statements::ModuleDeclaration { name } => m_name = name,
            _ => panic!(""),
        }
    }

    let context = Context::create();
    let module = context.create_module(&m_name);

    let llvm = LLVMContext {
        context: &context,
        module,
        builder: context.create_builder(),
    };

    let program = Program::init(ast);

    program.generate(&llvm, format!("{}.obj", output));

    cc::Build::new()
        .file(format!("{}.obj", output))
        .target(&llvm.module.get_triple().to_string())
        .opt_level(2)
        .host("x86_64-linux-gnu")
        .out_dir("./")
        .compile(&output);
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
