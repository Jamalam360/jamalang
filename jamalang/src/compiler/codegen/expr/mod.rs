use crate::parser::expr::{
    Array, ArrayIndex, BinOp, Bool, Char, Expr, Float, FunctionCall, Identifier, UnaryMinus,
};

use super::{value::Value, Codegen};

mod array;
mod function_call;
mod op;
mod values;

impl<'a, 'ctx> Codegen<'a, 'ctx> for Expr {
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
    ) -> super::value::Value<'ctx> {
        match self {
            Expr::Float(_) => {
                let value: Float = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Expr::Bool(_) => {
                let value: Bool = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Expr::Char(_) => {
                let value: Char = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Expr::TypeNone => Value::Void,
            Expr::Identifier(_) => {
                let value: Identifier = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Expr::FunctionCall { .. } => {
                let value: FunctionCall = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Expr::UnaryMinus(_) => {
                let value: UnaryMinus = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Expr::Array(_) => {
                let value: Array = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Expr::ArrayIndex { .. } => {
                let value: ArrayIndex = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Expr::BinOp { .. } => {
                let value: BinOp = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
        }
    }
}
