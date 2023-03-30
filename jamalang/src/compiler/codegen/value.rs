use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{ArrayValue, BasicValueEnum, FloatValue, GlobalValue, IntValue, PointerValue},
};

#[derive(Debug, Copy, Clone)]
pub enum Value<'ctx> {
    Float(FloatValue<'ctx>),
    Bool(IntValue<'ctx>),
    Char(IntValue<'ctx>),
    Array(ArrayValue<'ctx>),
    Void,
}

impl<'a, 'ctx> Value<'ctx> {
    pub fn alloca(
        &self,
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        identifier: &str,
    ) -> PointerValue<'ctx> {
        match self {
            Value::Float(_) => {
                builder.build_alloca(context.f32_type(), &format!("alloca_{}", identifier))
            }
            Value::Bool(_) => {
                builder.build_alloca(context.bool_type(), &format!("alloca_{}", identifier))
            }
            Value::Char(_) => {
                builder.build_alloca(context.i8_type(), &format!("alloca_{}", identifier))
            }
            Value::Array(array) => builder.build_array_alloca(
                array.get_type(),
                context
                    .i32_type()
                    .const_int(array.get_type().len() as u64, false),
                &format!("alloca_{}", identifier),
            ),
            Value::Void => panic!("Cannot allocate void"),
        }
    }

    pub fn global_alloca(
        &self,
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        identifier: &str,
    ) -> GlobalValue<'ctx> {
        match self {
            Value::Float(_) => {
                module.add_global(context.f32_type(), None, &format!("global_{}", identifier))
            }
            Value::Bool(_) => {
                module.add_global(context.bool_type(), None, &format!("global_{}", identifier))
            }
            Value::Char(_) => {
                module.add_global(context.i8_type(), None, &format!("global_{}", identifier))
            }
            Value::Array(array) => {
                module.add_global(array.get_type(), None, &format!("global_{}", identifier))
            }
            Value::Void => panic!("Cannot allocate void"),
        }
    }

    pub fn into_basic_value(self) -> BasicValueEnum<'ctx> {
        match self {
            Value::Float(float_value) => BasicValueEnum::FloatValue(float_value),
            Value::Bool(bool_value) => BasicValueEnum::IntValue(bool_value),
            Value::Char(char_value) => BasicValueEnum::IntValue(char_value),
            Value::Array(array_value) => BasicValueEnum::ArrayValue(array_value),
            Value::Void => panic!("Cannot convert void to basic value"),
        }
    }
}

impl<'ctx> From<BasicValueEnum<'ctx>> for Value<'ctx> {
    fn from(value: BasicValueEnum<'ctx>) -> Self {
        match value {
            BasicValueEnum::FloatValue(float_value) => Value::Float(float_value),
            BasicValueEnum::IntValue(int_value) => {
                if int_value.get_type().get_bit_width() == 1 {
                    Value::Bool(int_value)
                } else {
                    Value::Char(int_value)
                }
            }
            BasicValueEnum::ArrayValue(array_value) => Value::Array(array_value),
            _ => unreachable!("Unexpected basic value {:#?}", value),
        }
    }
}
