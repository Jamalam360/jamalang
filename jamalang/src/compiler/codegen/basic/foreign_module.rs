use crate::parser::ast::ForeignModule;

use crate::compiler::codegen::{value::Value, Codegen};

impl<'a, 'ctx> Codegen<'a, 'ctx> for ForeignModule {
    fn codegen(
        self,
        compiler: &'a crate::compiler::Compiler<'a, 'ctx>,
        main_function: bool,
        context: &'ctx inkwell::context::Context,
        module: &'a inkwell::module::Module<'ctx>,
        builder: &'a inkwell::builder::Builder<'ctx>,
        variables: &'a mut Vec<
            std::collections::HashMap<
                String,
                (
                    inkwell::values::BasicValueEnum<'ctx>,
                    inkwell::values::PointerValue<'ctx>,
                ),
            >,
        >,
    ) -> Value<'ctx> {
        //TODO: These should be compiled into a separate module
        for statement in self.0.statements {
            statement.codegen(compiler, main_function, context, module, builder, variables);
        }

        Value::Void
    }
}
