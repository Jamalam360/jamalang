use std::collections::HashMap;

use crate::parser::ast::{
    Assignment, Expr, ForLoop, ForeignModule, FunctionDefinition, IfStatement, Return, Statement,
    Type, WhileLoop,
};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicMetadataTypeEnum, BasicTypeEnum},
    values::{BasicValueEnum, PointerValue},
};

use crate::compiler::Compiler;

use self::value::Value;

mod basic;
mod control_flow;
mod expr;
mod looping;
mod value;

macro_rules! stack_top {
    ($expr:ident) => {
        $expr
            .last_mut()
            .expect("scopes should hold at least one value")
    };
}

pub(crate) use stack_top;

pub(crate) fn get_variable<'a, 'ctx>(
    variables: &'a Vec<HashMap<String, (BasicValueEnum<'ctx>, PointerValue<'ctx>)>>,
    identifier: &str,
) -> &'a (BasicValueEnum<'ctx>, PointerValue<'ctx>) {
    for scope in variables.iter().rev() {
        if let Some(value) = scope.get(identifier) {
            return value;
        }
    }

    panic!("Attempted to access unknown variable {}", identifier)
}

pub(crate) fn get_any_type_from_type_hint<'ctx>(
    context: &'ctx Context,
    hint: Type,
) -> BasicTypeEnum<'ctx> {
    match hint {
        Type::Char => BasicTypeEnum::IntType(context.i8_type()),
        Type::Bool => BasicTypeEnum::IntType(context.bool_type()),
        Type::Number => BasicTypeEnum::FloatType(context.f32_type()),
        //TODO: Real void types?
        Type::Void => BasicTypeEnum::IntType(context.bool_type()),
        _ => todo!("Type {:#?} is not yet supported", hint),
    }
}

pub(crate) fn get_basic_type_metadata_from_type_hint<'ctx>(
    context: &'ctx Context,
    hint: Type,
) -> BasicMetadataTypeEnum<'ctx> {
    match hint {
        Type::Char => BasicMetadataTypeEnum::IntType(context.i8_type()),
        Type::Bool => BasicMetadataTypeEnum::IntType(context.bool_type()),
        Type::Number => BasicMetadataTypeEnum::FloatType(context.f32_type()),
        // Type::Array(array_type) => BasicMetadataTypeEnum::ArrayType(match get_basic_type_metadata_from_type_hint(context, array_type) {
        // BasicMetadataTypeEnum::ArrayType(array_type) => array_type.array_type(size)
        // }),
        _ => todo!("Type {:#?} is not yet supported", hint),
    }
}

pub(crate) trait Codegen<'a, 'ctx> {
    fn codegen(
        self,
        compiler: &'a Compiler<'a, 'ctx>,
        main_function: bool,
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        builder: &'a Builder<'ctx>,
        variables: &'a mut Vec<HashMap<String, (BasicValueEnum<'ctx>, PointerValue<'ctx>)>>,
    ) -> Value<'ctx>;
}

impl<'a, 'ctx> Codegen<'a, 'ctx> for Statement {
    fn codegen(
        self,
        compiler: &'a Compiler<'a, 'ctx>,
        main_function: bool,
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        builder: &'a Builder<'ctx>,
        variables: &'a mut Vec<HashMap<String, (BasicValueEnum<'ctx>, PointerValue<'ctx>)>>,
    ) -> Value<'ctx> {
        match self {
            Statement::Assignment { .. } => {
                let value: Assignment = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Statement::FunctionDefinition { .. } => {
                let value: FunctionDefinition = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Statement::Return { .. } => {
                let value: Return = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Statement::WhileLoop { .. } => {
                let value: WhileLoop = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Statement::ForLoop { .. } => {
                let value: ForLoop = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Statement::IfStatement { .. } => {
                let value: IfStatement = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Statement::ForeignModule(_) => {
                let value: ForeignModule = self.try_into().unwrap();
                value.codegen(compiler, main_function, context, module, builder, variables)
            }
            Statement::Expr { .. } => {
                let value: Expr = self.try_into().unwrap();
                value
                    .expr
                    .codegen(compiler, main_function, context, module, builder, variables)
            }
            Statement::Comment(_) => Value::Void,
        }
    }
}
