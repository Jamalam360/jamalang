use crate::parser::ast::ForLoop;
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    values::{ArrayValue, PointerValue},
};

use crate::compiler::codegen::{stack_top, value::Value, Codegen};

impl<'a, 'ctx> Codegen<'a, 'ctx> for ForLoop {
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
        let array = self
            .expression
            .clone()
            .codegen(compiler, main_function, context, module, builder, variables)
            .into_basic_value()
            .into_array_value();

        let index_ptr = builder.build_alloca(
            context.i32_type(),
            &format!("modifier_internal$array_index_{}", self.identifier),
        );
        builder.build_store(index_ptr, context.i32_type().const_int(0u64, false));

        let array_ptr = builder.build_alloca(array.get_type(), "array_alloc");
        builder.build_store(array_ptr, array);

        let current_block = builder
            .get_insert_block()
            .expect("insert block should be present");
        let function = current_block
            .get_parent()
            .expect("function should be present");
        let loop_block = context.append_basic_block(function, "loop");
        let merge_block = context.append_basic_block(function, "merge");
        loop_meta_instructions(
            context,
            builder,
            variables,
            &loop_block,
            &merge_block,
            &array,
            &array_ptr,
            &index_ptr,
            &self.identifier,
        );
        builder.position_at_end(loop_block);

        for statement in self.body {
            statement.codegen(compiler, false, context, module, builder, variables);
        }

        let index = builder.build_load(
            context.i32_type(),
            index_ptr,
            &format!("load_{}", self.identifier),
        );

        builder.build_store(
            index_ptr,
            builder.build_int_add(
                index.into_int_value(),
                context.i32_type().const_int(1, false),
                "add",
            ),
        );

        loop_meta_instructions(
            context,
            builder,
            variables,
            &loop_block,
            &merge_block,
            &array,
            &array_ptr,
            &index_ptr,
            &self.identifier,
        );
        builder.position_at_end(merge_block);

        Value::Void
    }
}

fn loop_meta_instructions<'a, 'ctx>(
    context: &'ctx Context,
    builder: &'a Builder<'ctx>,
    variables: &'a mut Vec<
        std::collections::HashMap<
            String,
            (
                inkwell::values::BasicValueEnum<'ctx>,
                inkwell::values::PointerValue<'ctx>,
            ),
        >,
    >,
    loop_block: &'a BasicBlock<'ctx>,
    exit_block: &'a BasicBlock<'ctx>,
    array: &'a ArrayValue<'ctx>,
    array_ptr: &'a PointerValue<'ctx>,
    index_ptr: &'a PointerValue<'ctx>,
    identifier: &String,
) {
    let index = builder
        .build_load(context.i32_type(), *index_ptr, "load_index")
        .into_int_value();
    let element_ptr = unsafe {
        builder.build_gep(
            array.get_type().get_element_type(),
            *array_ptr,
            &[index],
            "array_extract",
        )
    };

    let element = builder.build_load(
        array.get_type().get_element_type(),
        element_ptr,
        "indexing_load",
    );

    if let Some((_, ptr)) = stack_top!(variables).get(identifier) {
        builder.build_store(*ptr, element);
    } else {
        let ptr = builder.build_alloca(element.get_type(), &identifier);
        builder.build_store(ptr, element);
        stack_top!(variables).insert(identifier.clone(), (element, ptr));
    };

    let condition_value = builder.build_int_compare(
        inkwell::IntPredicate::NE,
        index,
        context
            .i32_type()
            .const_int(array.get_type().len() as u64, false),
        "for_cond",
    );

    builder.build_conditional_branch(condition_value, *loop_block, *exit_block);
}
