pub mod expressions;
pub mod statements;
use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuncParam {
    pub name: String,
    pub r#type: Type,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Float16,
    Float32,
    Float64,
    Float128,
    String,
    Boolean,
    Void,
    Array(Box<Type>),
    UserDefinedType { name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<statements::Statements>,
    pub exports: Vec<Export>,
}

impl Program {
    pub fn compile(app: (Vec<statements::Statements>, Vec<Export>)) -> Self {
        Self {
            statements: app.0,
            exports: app.1
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportType {
    Const,
    Function,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub r#type: ExportType,
    pub statement: statements::Statements
}

impl Type {
    pub fn parse_type(type_str: String) -> Type {
        let r#str = type_str.as_str();

        match r#str {
            "i8" => Type::Int8,
            "i16" => Type::Int16,
            "i32" => Type::Int32,
            "i64" => Type::Int64,
            "i128" => Type::Int128,
            "f16" => Type::Float16,
            "f32" => Type::Float32,
            "f64" => Type::Float64,
            "f128" => Type::Float128,
            "str" => Type::String,
            "bool" => Type::Boolean,
            "void" => Type::Void,
            name => {
                let regexp = Regex::new(r"array\[[a-zA-Z0-9_]+\]").unwrap();

                if regexp.is_match(name) {
                    let name = name.replace("[", " ");
                    let name = name.replace("]", "");
                    let xs = name.split(" ");
                    let mut str_arr: Vec<&str> = vec![];
                    for x in xs {
                        str_arr.push(x);
                    }
                    return Type::Array(Box::new(Type::parse_type(str_arr[1].to_owned())));
                }

                Type::UserDefinedType {
                    name: name.to_string(),
                }
            }
        }
    }
}


