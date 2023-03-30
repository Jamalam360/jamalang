use crate::parser::expr::{Array, ArrayIndex};

use crate::compiler::codegen::{value::Value, Codegen};

impl<'a, 'ctx> Codegen<'a, 'ctx> for Array {
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
        let mut values = vec![];
        for expr in self.0 {
            values.push(expr.codegen(compiler, main_function, context, module, builder, variables));
        }

        let array = match values
            .first()
            .expect("array should have at least one element")
        {
            Value::Float(float_value) => float_value.get_type().const_array(
                values
                    .iter()
                    .map(|value| value.into_basic_value().into_float_value())
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            Value::Char(char_value) => char_value.get_type().const_array(
                values
                    .iter()
                    .map(|value| value.into_basic_value().into_int_value())
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            Value::Bool(bool_value) => bool_value.get_type().const_array(
                values
                    .iter()
                    .map(|value| value.into_basic_value().into_int_value())
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            _ => unreachable!("Variable type not yet supported in arrays"),
        };

        Value::Array(array)
    }
}

impl<'a, 'ctx> Codegen<'a, 'ctx> for ArrayIndex {
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
            .array
            .codegen(compiler, main_function, context, module, builder, variables)
            .into_basic_value();

        let index =
            self.index
                .codegen(compiler, main_function, context, module, builder, variables);

        if let Value::Float(index) = index {
            let array_ptr = builder.build_alloca(array.get_type(), "indexing_alloc");
            builder.build_store(array_ptr, array);

            let ptr = unsafe {
                builder.build_gep(
                    array.into_array_value().get_type().get_element_type(),
                    array_ptr,
                    &[index.const_to_unsigned_int(context.i32_type())],
                    "array_extract",
                )
            };

            builder
                .build_load(
                    array.get_type().into_array_type().get_element_type(),
                    ptr,
                    "indexing_load",
                )
                .into()
        } else {
            panic!("Index is not a number")
        }
    }
}
