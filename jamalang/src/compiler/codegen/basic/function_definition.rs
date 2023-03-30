use std::collections::HashMap;

use crate::parser::ast::FunctionDefinition;
use inkwell::types::BasicType;

use crate::compiler::codegen::{
    get_any_type_from_type_hint, get_basic_type_metadata_from_type_hint, stack_top, value::Value,
    Codegen,
};

impl<'a, 'ctx> Codegen<'a, 'ctx> for FunctionDefinition {
    fn codegen(
        self,
        compiler: &'a crate::compiler::Compiler<'a, 'ctx>,
        _main_function: bool,
        context: &'ctx inkwell::context::Context,
        module: &'a inkwell::module::Module<'ctx>,
        _builder: &'a inkwell::builder::Builder<'ctx>,
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
        let return_type = get_any_type_from_type_hint(context, self.return_type_hint);
        let fn_type = return_type.fn_type(
            self.parameters
                .iter()
                .map(|p| get_basic_type_metadata_from_type_hint(context, p.1.clone()))
                .collect::<Vec<_>>()
                .as_slice(),
            false,
        );

        let function = module.add_function(self.identifier.as_str(), fn_type, None);
        let fn_builder = context.create_builder();
        fn_builder.position_at_end(context.append_basic_block(function, self.identifier.as_str()));
        variables.push(HashMap::new());

        for (idx, parameter) in self.parameters.into_iter().enumerate() {
            let value = function.get_nth_param(idx as u32);

            if let Some(value) = value {
                let value: Value = value.into();
                let ptr = value.alloca(context, &fn_builder, &parameter.0);
                fn_builder.build_store(ptr, value.into_basic_value());
                stack_top!(variables).insert(parameter.0, (value.into_basic_value(), ptr));
            } else {
                panic!(
                    "Could not get parameter of index {} for function {}",
                    idx, self.identifier
                );
            }
        }

        for statement in self.body {
            statement.codegen(compiler, false, context, module, &fn_builder, variables);
        }

        if fn_builder
            .get_insert_block()
            .expect("insert block should be present")
            .get_terminator()
            .is_none()
        {
            fn_builder.build_return(None);
        }

        variables.pop();

        Value::Void
    }
}
