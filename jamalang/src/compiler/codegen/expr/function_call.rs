use either::Either;
use inkwell::values::BasicMetadataValueEnum;
use crate::parser::expr::FunctionCall;

use crate::compiler::codegen::{value::Value, Codegen};

impl<'a, 'ctx> Codegen<'a, 'ctx> for FunctionCall {
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
    ) -> crate::compiler::codegen::Value<'ctx> {
        let mut args = vec![];
        for parameter in self.parameters {
            let value =
                parameter.codegen(compiler, main_function, context, module, builder, variables);
            args.push(match value {
                Value::Float(float_value) => BasicMetadataValueEnum::FloatValue(float_value),
                Value::Char(char_value) => BasicMetadataValueEnum::IntValue(char_value),
                Value::Bool(bool_value) => BasicMetadataValueEnum::IntValue(bool_value),
                _ => unreachable!(
                    "Variable type {:#?} not yet supported in function calls",
                    value
                ),
            });
        }

        let mut function = module.get_function(&self.identifier);

        if function.is_none() {
            let typed_identifier = format!(
                "{}_{}",
                self.identifier,
                args.iter()
                    .map(|a| match a {
                        BasicMetadataValueEnum::FloatValue(_) => "n",
                        BasicMetadataValueEnum::IntValue(int_value) => {
                            if int_value.get_type().get_bit_width() == 1 {
                                "b"
                            } else {
                                "c"
                            }
                        }
                        _ => panic!("Cannot convert {:?} to signature", a),
                    })
                    .collect::<String>()
            );

            function = module.get_function(&typed_identifier);
        }

        let ret = builder.build_direct_call(
            function.expect("function should be present"),
            args.as_slice(),
            "call",
        );

        match ret.try_as_basic_value() {
            Either::Left(value) => value.into(),
            Either::Right(_) => Value::Void,
        }
    }
}
