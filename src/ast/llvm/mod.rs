use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    types::BasicMetadataTypeEnum,
    AddressSpace, OptimizationLevel,
};

use super::statements::Statements;

pub struct LLVMContext<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl<'ctx> LLVMContext<'ctx> {
    pub fn create_var(&self, statement: Statements) {
        match statement {
            Statements::VariableDeclaration { name, .. } => {
                let type_ = self.context.i8_type();

                self.module
                    .add_global(type_, Some(AddressSpace::from(1)), &name);
            }
            Statements::ModuleDeclaration { .. } => unreachable!(),
            Statements::FunctionDeclaration { .. } => unreachable!(),
        }
    }

    pub fn create_function(&self, statement: Statements) {
        match statement {
            Statements::FunctionDeclaration {
                name,
                params,
                return_type,
                ..
            } => {
                let params_len: usize = params.len();
                let mut fn_params: Vec<BasicMetadataTypeEnum> = Vec::with_capacity(params_len);
                let mut void: bool = false;


                for param in params {
                    match param.r#type {
                        // Ints
                        super::Type::Int8 => {
                            fn_params.push(BasicMetadataTypeEnum::IntType(self.context.i8_type()))
                        }
                        super::Type::Int16 => {
                            fn_params.push(BasicMetadataTypeEnum::IntType(self.context.i16_type()))
                        }
                        super::Type::Int32 => {
                            fn_params.push(BasicMetadataTypeEnum::IntType(self.context.i32_type()))
                        }
                        super::Type::Int64 => {
                            fn_params.push(BasicMetadataTypeEnum::IntType(self.context.i64_type()))
                        }
                        super::Type::Int128 => {
                            fn_params.push(BasicMetadataTypeEnum::IntType(self.context.i128_type()))
                        }
                        // Floats
                        super::Type::Float16 => fn_params
                            .push(BasicMetadataTypeEnum::FloatType(self.context.f16_type())),
                        super::Type::Float32 => fn_params
                            .push(BasicMetadataTypeEnum::FloatType(self.context.f32_type())),
                        super::Type::Float64 => fn_params
                            .push(BasicMetadataTypeEnum::FloatType(self.context.f64_type())),
                        super::Type::Float128 => fn_params
                            .push(BasicMetadataTypeEnum::FloatType(self.context.f128_type())),
                        super::Type::String => fn_params.push(BasicMetadataTypeEnum::PointerType(
                            self.context.i8_type().ptr_type(AddressSpace::default()),
                        )),
                        super::Type::Void => void = true,
                        super::Type::Boolean => todo!(),
                        super::Type::Array(_) => todo!(),
                        super::Type::UserDefinedType { .. } => todo!(),
                    }
                }

                let params = fn_params.as_slice();

                let fn_type = if void {
                    match return_type {
                        // Ints
                        super::Type::Int8 => self.context.i8_type().fn_type(params, false),
                        super::Type::Int16 => self.context.i16_type().fn_type(params, false),
                        super::Type::Int32 => self.context.i32_type().fn_type(params, false),
                        super::Type::Int64 => self.context.i64_type().fn_type(params, false),
                        super::Type::Int128 => self.context.i128_type().fn_type(params, false),
                        // Floats
                        super::Type::Float16 => self.context.f16_type().fn_type(params, false),
                        super::Type::Float32 => self.context.f32_type().fn_type(params, false),
                        super::Type::Float64 => self.context.f64_type().fn_type(params, false),
                        super::Type::Float128 => self.context.f128_type().fn_type(params, false),
                        super::Type::String => self
                            .context
                            .i8_type()
                            .ptr_type(AddressSpace::default())
                            .fn_type(params, false),
                        super::Type::Void => self.context.void_type().fn_type(params, false),
                        super::Type::Boolean => todo!(),
                        super::Type::Array(_) => todo!(),
                        super::Type::UserDefinedType { .. } => todo!(),
                    }
                } else {
                    self.context.void_type().fn_type(params, false)
                };

                let fn_val = self.module.add_function(name.as_str(), fn_type, None);
                let basic_block = self.context.append_basic_block(fn_val, "last");
                // self.builder.position_at_end(basic_block);
            }
            _ => unreachable!(),
        }
    }

    pub fn build(&self, output: String) {
        Target::initialize_all(&InitializationConfig::default());
        // use the host machine as the compilation target
        let target_triple = TargetMachine::get_default_triple();
        let cpu = TargetMachine::get_host_cpu_name().to_string();
        let features = TargetMachine::get_host_cpu_features().to_string();

        // make a target from the triple
        let target = Target::from_triple(&target_triple)
            .map_err(|e| format!("{:?}", e))
            .unwrap();

        // make a machine from the target
        let target_machine = target
            .create_target_machine(
                &target_triple,
                &cpu,
                &features,
                OptimizationLevel::Default,
                RelocMode::Default,
                CodeModel::Default,
            )
            .ok_or_else(|| "Unable to create target machine!".to_string())
            .unwrap();

        // use the machine to convert our module to machine code and write the result to a file
        target_machine
            .write_to_file(&self.module, FileType::Object, output.as_ref())
            .map_err(|e| format!("{:?}", e))
            .unwrap();
    }
}

pub struct Program {
    ast: Vec<Statements>,
}

impl Program {
    pub fn init(ast: Vec<Statements>) -> Self {
        Self { ast }
    }

    pub fn generate(&self, ctx: &LLVMContext, output: String) {
        for ins in &self.ast {
            match ins {
                Statements::VariableDeclaration { .. } => ctx.create_var(ins.clone()),
                Statements::ModuleDeclaration { .. } => todo!(),
                Statements::FunctionDeclaration { .. } => ctx.create_function(ins.clone()),
            }
        }

        // ctx.temp();

        ctx.build(output)
    }
}
