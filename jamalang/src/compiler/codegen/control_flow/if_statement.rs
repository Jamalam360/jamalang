use crate::parser::ast::IfStatement;

use crate::compiler::codegen::{value::Value, Codegen};

impl<'a, 'ctx> Codegen<'a, 'ctx> for IfStatement {
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
        let condition = self
            .condition
            .codegen(compiler, main_function, context, module, builder, variables)
            .into_basic_value();
        let current_block = builder
            .get_insert_block()
            .expect("insert block should be present");
        let function = current_block
            .get_parent()
            .expect("function should be present");
        let then_block = context.append_basic_block(function, "then");
        let else_block = context.append_basic_block(function, "else");
        let merge_block = context.append_basic_block(function, "merge");

        builder.build_conditional_branch(condition.into_int_value(), then_block, else_block);

        builder.position_at_end(then_block);

        for statement in self.body {
            statement.codegen(compiler, false, context, module, builder, variables);
        }

        if builder
            .get_insert_block()
            .expect("insert block should be present")
            .get_terminator()
            .is_none()
        {
            builder.build_unconditional_branch(merge_block);
        }

        builder.position_at_end(else_block);

        for (condition, body) in self.else_ifs {
            let condition = condition
                .codegen(compiler, main_function, context, module, builder, variables)
                .into_basic_value();
            let current_block = builder
                .get_insert_block()
                .expect("insert block should be present");
            let function = current_block
                .get_parent()
                .expect("function should be present");
            let then_block = context.append_basic_block(function, "then");
            let else_block = context.append_basic_block(function, "else");

            builder.build_conditional_branch(condition.into_int_value(), then_block, else_block);

            builder.position_at_end(then_block);

            for statement in body {
                statement.codegen(compiler, false, context, module, builder, variables);
            }

            if builder
                .get_insert_block()
                .expect("insert block should be present")
                .get_terminator()
                .is_none()
            {
                builder.build_unconditional_branch(merge_block);
            }

            builder.position_at_end(else_block);
        }

        for statement in self.else_body {
            statement.codegen(compiler, false, context, module, builder, variables);
        }

        if builder
            .get_insert_block()
            .expect("insert block should be present")
            .get_terminator()
            .is_none()
        {
            builder.build_unconditional_branch(merge_block);
        }

        builder.position_at_end(merge_block);

        Value::Void
    }
}
