use inkwell::{
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
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
                body,
                return_type,
            } => {

                // let params_len = params.len();

                // let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
                // let fn_val = module.add_function("add", fn_type, None);
                // let entry_basic_block = self.context.append_basic_block(fn_val, "entry");

                // self.module.add_function(&name, ty, None);
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

    pub fn temp(&self) {
        let i32_type = self.context.i32_type();
        let main_fn_type = i32_type.fn_type(&[], false);
        let main_fn = self
            .module
            .add_function("main", main_fn_type, Some(Linkage::External));

        let basic_block = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(basic_block);

        let i32_zero = i32_type.const_int(69, false);
        self.builder.build_return(Some(&i32_zero));
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
        // for ins in &self.ast {
        //     match ins {
        //         Statements::VariableDeclaration { .. } => ctx.create_var(ins.clone()),
        //         Statements::ModuleDeclaration { .. } => todo!(),
        //         Statements::FunctionDeclaration { .. } => ctx.create_function(ins.clone()),
        //     }
        // }

        ctx.temp();

        ctx.build(output)
    }
}
