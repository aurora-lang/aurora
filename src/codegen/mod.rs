use aurorac_macros::generate_params;
use inkwell::{
    context::ContextRef,
    module::Module,
    types::{BasicMetadataTypeEnum, FunctionType},
};

use crate::ast::{statements::Statements, FuncParam, Type};

pub fn get_param_type(r#type: Type) -> String {
    match r#type {
        Type::Int8 => "into_int_value",
        Type::Int16 => "into_int_value",
        Type::Int32 => "into_int_value",
        Type::Int64 => "into_int_value",
        Type::Int128 => "into_int_value",
        Type::Float16 => "into_float_value",
        Type::Float32 => "into_float_value",
        Type::Float64 => "into_float_value",
        Type::Float128 => "into_float_value",
        Type::String => todo!(),
        Type::Boolean => "into_int_value",
        Type::Void => unreachable!(),
        Type::Array(_) => "into_array_value",
        Type::UserDefinedType { name } => todo!(),
    }
    .to_string()
}

pub fn get_function_params<'a>(
    params: Vec<FuncParam>,
    ctx: &ContextRef<'a>,
) -> Vec<BasicMetadataTypeEnum<'a>> {
    let mut llvm_params: Vec<BasicMetadataTypeEnum> = vec![];

    let i8_type = ctx.i8_type().clone();
    let i16_type = ctx.i16_type().clone();
    let i32_type = ctx.i32_type().clone();
    let i64_type = ctx.i64_type().clone();
    let i128_type = ctx.i128_type().clone();

    let f16_type = ctx.f16_type().clone();
    let f32_type = ctx.f32_type().clone();
    let f64_type = ctx.f64_type().clone();
    let f128_type = ctx.f128_type().clone();

    let bool_type = ctx.bool_type().clone();

    for param in params {
        match param.r#type {
            Type::Int8 => llvm_params.push(i8_type.into()),
            Type::Int16 => llvm_params.push(i16_type.into()),
            Type::Int32 => llvm_params.push(i32_type.into()),
            Type::Int64 => llvm_params.push(i64_type.into()),
            Type::Int128 => llvm_params.push(i128_type.into()),
            Type::Float16 => llvm_params.push(f16_type.into()),
            Type::Float32 => llvm_params.push(f32_type.into()),
            Type::Float64 => llvm_params.push(f64_type.into()),
            Type::Float128 => llvm_params.push(f128_type.into()),
            Type::String => todo!(),
            Type::Boolean => llvm_params.push(bool_type.into()),
            Type::Void => panic!("Function param type can not be void"),
            Type::Array(_) => todo!(),
            Type::UserDefinedType { .. } => todo!(),
        }
    }

    llvm_params
}

pub fn get_function_type<'a>(
    ret_type: Type,
    params: Vec<BasicMetadataTypeEnum<'a>>,
    ctx: &ContextRef<'a>,
) -> FunctionType<'a> {
    let fn_type = match ret_type {
        Type::Int8 => ctx.i8_type().fn_type(&params, false).clone(),
        Type::Int16 => ctx.i16_type().fn_type(&params, false).clone(),
        Type::Int32 => ctx.i32_type().fn_type(&params, false).clone(),
        Type::Int64 => ctx.i64_type().fn_type(&params, false).clone(),
        Type::Int128 => ctx.i128_type().fn_type(&params, false).clone(),
        Type::Float16 => ctx.f16_type().fn_type(&params, false).clone(),
        Type::Float32 => ctx.i32_type().fn_type(&params, false).clone(),
        Type::Float64 => ctx.i64_type().fn_type(&params, false).clone(),
        Type::Float128 => ctx.i128_type().fn_type(&params, false).clone(),
        Type::String => {
            todo!()
        }
        Type::Boolean => ctx.bool_type().fn_type(&params, false).clone(),
        Type::Void => ctx.void_type().fn_type(&params, false).clone(),
        Type::Array(_) => {
            todo!()
        }
        Type::UserDefinedType { .. } => {
            todo!()
        }
    };

    fn_type
}

pub struct CodeGen;

impl CodeGen {
    pub fn new() -> Self {
        Self
    }

    pub fn compile_function_statement(
        &self,
        function: Statements,
        r#mod: &Module,
    ) -> Result<(), String> {
        match function {
            Statements::FunctionDeclaration {
                name,
                params,
                body,
                return_type,
            } => {
                let ctx = r#mod.get_context();

                let fn_params = get_function_params(params.clone(), &ctx);
                let fn_type = get_function_type(return_type, fn_params, &ctx);
                let fn_val = r#mod.add_function(&name, fn_type, None);
                let entry_basic_block = ctx.append_basic_block(fn_val, "entry");

                let builder = ctx.create_builder();
                builder.position_at_end(entry_basic_block);

                let mut curr_param = 0;
                for param in params {
                    let FuncParam { name, r#type } = param;
                    let param_type = r#type.to();
                    let param_type = get_param_type(Type::parse_type(param_type.to_string()));
                    generate_params!(name curr_param param_type);
                    curr_param += 1;
                }

                let x = fn_val.get_nth_param(0).unwrap().into_int_value();
                let y = fn_val.get_nth_param(1).unwrap().into_int_value();

                let ret = builder.build_int_add(x, y, "add");
                let _ = builder.build_return(Some(&ret));

                return Ok(());
            }
            _ => return Err(format!("Expected FunctionDecl as input")),
        }
    }
}
