use crate::parser::ast::WhileLoop;

use crate::compiler::codegen::{value::Value, Codegen};

impl<'a, 'ctx> Codegen<'a, 'ctx> for WhileLoop {
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
        let condition_value = self
            .condition
            .clone()
            .codegen(compiler, main_function, context, module, builder, variables)
            .into_basic_value();
        let current_block = builder
            .get_insert_block()
            .expect("insert block should be present");
        let function = current_block
            .get_parent()
            .expect("function should be present");

        let loop_block = context.append_basic_block(function, "loop");
        let merge_block = context.append_basic_block(function, "merge");
        builder.build_conditional_branch(condition_value.into_int_value(), loop_block, merge_block);
        builder.position_at_end(loop_block);

        for statement in self.body {
            statement.codegen(compiler, false, context, module, builder, variables);
        }

        let condition_value = self
            .condition
            .codegen(compiler, main_function, context, module, builder, variables)
            .into_basic_value();
        builder.build_conditional_branch(condition_value.into_int_value(), loop_block, merge_block);
        builder.position_at_end(merge_block);

        Value::Void
    }
}
