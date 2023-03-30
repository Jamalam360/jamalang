use crate::parser::expr::{Bool, Char, Float, Identifier};

use crate::compiler::codegen::{get_variable, value::Value, Codegen};

impl<'a, 'ctx> Codegen<'a, 'ctx> for Float {
    fn codegen(
        self,
        _: &'a crate::compiler::Compiler<'a, 'ctx>,
        _: bool,
        context: &'ctx inkwell::context::Context,
        _: &'a inkwell::module::Module<'ctx>,
        _: &'a inkwell::builder::Builder<'ctx>,
        _: &'a mut Vec<
            std::collections::HashMap<
                String,
                (
                    inkwell::values::BasicValueEnum<'ctx>,
                    inkwell::values::PointerValue<'ctx>,
                ),
            >,
        >,
    ) -> crate::compiler::codegen::Value<'ctx> {
        Value::Float(context.f32_type().const_float(self.0 as f64))
    }
}

impl<'a, 'ctx> Codegen<'a, 'ctx> for Char {
    fn codegen(
        self,
        _: &'a crate::compiler::Compiler<'a, 'ctx>,
        _: bool,
        context: &'ctx inkwell::context::Context,
        _: &'a inkwell::module::Module<'ctx>,
        _: &'a inkwell::builder::Builder<'ctx>,
        _: &'a mut Vec<
            std::collections::HashMap<
                String,
                (
                    inkwell::values::BasicValueEnum<'ctx>,
                    inkwell::values::PointerValue<'ctx>,
                ),
            >,
        >,
    ) -> crate::compiler::codegen::Value<'ctx> {
        Value::Char(context.i8_type().const_int(self.0 as u64, false))
    }
}

impl<'a, 'ctx> Codegen<'a, 'ctx> for Bool {
    fn codegen(
        self,
        _: &'a crate::compiler::Compiler<'a, 'ctx>,
        _: bool,
        context: &'ctx inkwell::context::Context,
        _: &'a inkwell::module::Module<'ctx>,
        _: &'a inkwell::builder::Builder<'ctx>,
        _: &'a mut Vec<
            std::collections::HashMap<
                String,
                (
                    inkwell::values::BasicValueEnum<'ctx>,
                    inkwell::values::PointerValue<'ctx>,
                ),
            >,
        >,
    ) -> crate::compiler::codegen::Value<'ctx> {
        Value::Bool(context.bool_type().const_int(self.0 as u64, false))
    }
}

impl<'a, 'ctx> Codegen<'a, 'ctx> for Identifier {
    fn codegen(
        self,
        _: &'a crate::compiler::Compiler<'a, 'ctx>,
        _: bool,
        _: &'ctx inkwell::context::Context,
        _: &'a inkwell::module::Module<'ctx>,
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
        let (ty, ptr) = get_variable(variables, &self.0);
        builder
            .build_load(ty.get_type(), *ptr, &format!("load_{}", self.0))
            .into()
    }
}
