pub mod expressions;
pub mod statements;
use regex::Regex;


#[derive(Debug, Clone)]
pub struct FuncParam {
    pub name: String,
    pub r#type: Type
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    Boolean,
    Array(Box<Type>),
    UserDefinedType { name: String },
}

impl Type {
    pub fn parse_type(type_str: String) -> Type {
        let r#str = type_str.as_str();

        match r#str {
            "int" => Type::Int,
            "float" => Type::Float,
            "str" => Type::String,
            "bool" => Type::Boolean,
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
