use std::{collections::HashMap, ffi::c_char};

use crate::compiler::codegen::Codegen;
use crate::parser::ModifierFile;
use builtins::{builtins, Builtin};
use inkwell::{
    builder::Builder,
    context::Context,
    memory_buffer::MemoryBuffer,
    module::{Linkage, Module},
    values::{BasicValueEnum, PointerValue},
};
use llvm_sys::support::LLVMAddSymbol;

mod builtins;
mod codegen;

const STDLIB: &[u8] = include_bytes!("../stdlib.bc");

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub builtins: Vec<Builtin<'ctx>>,
}

pub struct CompileResult<'ctx> {
    pub variables: HashMap<String, (BasicValueEnum<'ctx>, PointerValue<'ctx>)>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        builder: &'a Builder<'ctx>,
    ) -> Self {
        Self {
            context,
            builder,
            module,
            builtins: builtins(context),
        }
    }

    pub fn compile(&mut self, file: ModifierFile) -> CompileResult {
        for builtin in &self.builtins {
            unsafe { LLVMAddSymbol(builtin.name.as_ptr() as *const c_char, builtin.function) }
            self.module
                .add_function(&builtin.name, builtin.f_type, Some(Linkage::External));
        }

        self.link_stdlib();

        let main = self.module.add_function(
            &format!("{}_entry", file.source_file),
            self.context.void_type().fn_type(&[], false),
            None,
        );
        let block = self.context.append_basic_block(main, "entry");
        self.builder.position_at_end(block);
        let mut variables = vec![HashMap::new()];

        for statement in file.statements {
            statement.codegen(
                self,
                true,
                self.context,
                self.module,
                self.builder,
                &mut variables,
            );
        }

        self.builder.build_return(None);

        CompileResult {
            variables: variables
                .pop()
                .expect("scopes should hold at least 1 value"),
        }
    }

    fn link_stdlib(&self) {
        let memory_buffer = MemoryBuffer::create_from_memory_range(STDLIB, "stdlib");
        let module = self
            .module
            .get_context()
            .create_module_from_ir(memory_buffer)
            .expect("stdlib should be valid");
        self.module
            .link_in_module(module)
            .expect("stdlib should be linkable");
    }
}
