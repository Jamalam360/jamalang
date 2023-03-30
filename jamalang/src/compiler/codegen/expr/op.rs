use inkwell::FloatPredicate;
use crate::parser::expr::{BinOp, Op, UnaryMinus};

use crate::compiler::codegen::{value::Value, Codegen};

impl<'a, 'ctx> Codegen<'a, 'ctx> for UnaryMinus {
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
        let value = self
            .0
            .codegen(compiler, main_function, context, module, builder, variables);
        match value {
            Value::Float(float_value) => Value::Float(builder.build_float_neg(float_value, "neg")),
            _ => unreachable!("Cannot perform unary minus on {:#?}", value),
        }
    }
}

impl<'a, 'ctx> Codegen<'a, 'ctx> for BinOp {
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
        let lhs = self
            .lhs
            .codegen(compiler, main_function, context, module, builder, variables);
        let rhs = self
            .rhs
            .codegen(compiler, main_function, context, module, builder, variables);

        match self.op {
            Op::Add => match (lhs, rhs) {
                (Value::Float(lhs), Value::Float(rhs)) => {
                    Value::Float(builder.build_float_add(lhs, rhs, "add"))
                }
                _ => todo!("Cannot yet add {:?} and {:?}", lhs, rhs),
            },
            Op::Subtract => match (lhs, rhs) {
                (Value::Float(lhs), Value::Float(rhs)) => {
                    Value::Float(builder.build_float_sub(lhs, rhs, "sub"))
                }
                _ => todo!("Cannot yet subtract {:?} and {:?}", lhs, rhs),
            },
            Op::Multiply => match (lhs, rhs) {
                (Value::Float(lhs), Value::Float(rhs)) => {
                    Value::Float(builder.build_float_mul(lhs, rhs, "mul"))
                }
                _ => todo!("Cannot yet multiply {:?} and {:?}", lhs, rhs),
            },
            Op::Divide => match (lhs, rhs) {
                (Value::Float(lhs), Value::Float(rhs)) => {
                    Value::Float(builder.build_float_div(lhs, rhs, "div"))
                }
                _ => todo!("Cannot yet divide {:?} and {:?}", lhs, rhs),
            },
            Op::Modulo => match (lhs, rhs) {
                (Value::Float(lhs), Value::Float(rhs)) => {
                    Value::Float(builder.build_float_rem(lhs, rhs, "rem"))
                }
                _ => todo!("Cannot yet modulo {:?} and {:?}", lhs, rhs),
            },
            Op::Power => todo!("Power operator not implemented"),
            Op::Range => match (lhs, rhs) {
                (Value::Float(lhs), Value::Float(rhs)) => {
                    let start =  lhs.get_constant().expect("range expressions should currently be constant").0 as u32;
                    let end =  rhs.get_constant().expect("range expressions should currently be constant").0 as u32;
                    Value::Array(context.f32_type().const_array((start..end).map(|e| context.f32_type().const_float(e as f64)).collect::<Vec<_>>().as_slice()))
                }
                _ => todo!("Cannot yet range {:?} and {:?}", lhs, rhs),
            },
            Op::Lt => match (lhs, rhs) {
                (Value::Float(lhs), Value::Float(rhs)) => {
                    let cmp = builder.build_float_compare(FloatPredicate::ULT, lhs, rhs, "cmp");
                    Value::Bool(cmp)
                }
                _ => todo!("Cannot yet LT {:?} and {:?}", lhs, rhs),
            },
            Op::Gt => match (lhs, rhs) {
                (Value::Float(lhs), Value::Float(rhs)) => {
                    let cmp = builder.build_float_compare(FloatPredicate::UGT, lhs, rhs, "cmp");
                    Value::Bool(cmp)
                }
                _ => todo!("Cannot yet GT {:?} and {:?}", lhs, rhs),
            },
            Op::Lte => match (lhs, rhs) {
                (Value::Float(lhs), Value::Float(rhs)) => {
                    let cmp = builder.build_float_compare(FloatPredicate::ULE, lhs, rhs, "cmp");
                    Value::Bool(cmp)
                }
                _ => todo!("Cannot yet LTE {:?} and {:?}", lhs, rhs),
            },
            Op::Gte => match (lhs, rhs) {
                (Value::Float(lhs), Value::Float(rhs)) => {
                    let cmp = builder.build_float_compare(FloatPredicate::UGE, lhs, rhs, "cmp");
                    Value::Bool(cmp)
                }
                _ => todo!("Cannot yet GTE {:?} and {:?}", lhs, rhs),
            },
            Op::Eq => match (lhs, rhs) {
                (Value::Float(lhs), Value::Float(rhs)) => {
                    let cmp = builder.build_float_compare(FloatPredicate::UEQ, lhs, rhs, "cmp");
                    Value::Bool(cmp)
                }
                _ => todo!("Cannot yet EQ {:?} and {:?}", lhs, rhs),
            },
            Op::NotEq => match (lhs, rhs) {
                (Value::Float(lhs), Value::Float(rhs)) => {
                    let cmp = builder.build_float_compare(FloatPredicate::UNE, lhs, rhs, "cmp");
                    Value::Bool(cmp)
                }
                _ => todo!("Cannot yet NEQ {:?} and {:?}", lhs, rhs),
            },
        }
    }
}
