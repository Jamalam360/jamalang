use crate::parser::ast::{Assignment, AssignmentType};
use inkwell::values::{BasicValue, BasicValueEnum};

use crate::compiler::codegen::{get_variable, stack_top, value::Value, Codegen};

impl<'a, 'ctx> Codegen<'a, 'ctx> for Assignment {
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
        let ass_value =
            self.value
                .codegen(compiler, main_function, context, module, builder, variables);
        if let Some(index) = self.index {
            let index = index.codegen(compiler, main_function, context, module, builder, variables);
            match self.r#type {
                AssignmentType::Set => {
                    let (array, array_ptr) = get_variable(variables, &self.identifier);

                    if let Value::Float(index) = index {
                        let ptr = unsafe {
                            builder.build_gep(
                                array.into_array_value().get_type().get_element_type(),
                                *array_ptr,
                                &[index.const_to_unsigned_int(context.i32_type())],
                                "array_extract",
                            )
                        };

                        builder.build_store(ptr, ass_value.into_basic_value());
                    } else {
                        panic!("Index is not a number")
                    }
                }
                _ => unreachable!("Cannot yet use {:?} on arrays", self.r#type),
            }
        } else {
            match self.r#type {
                AssignmentType::Set => {
                    let ptr = if main_function {
                        let global = ass_value.global_alloca(context, module, &self.identifier);
                        global.set_initializer(&ass_value.into_basic_value());
                        global.as_pointer_value()
                    } else {
                        let ptr = ass_value.alloca(context, builder, &self.identifier);
                        builder.build_store(ptr, ass_value.into_basic_value());
                        ptr
                    };

                    stack_top!(variables)
                        .insert(self.identifier, (ass_value.into_basic_value(), ptr));
                }
                AssignmentType::Add => {
                    let (ty, ptr) = get_variable(variables, &self.identifier);
                    let current_value = builder.build_load(
                        ty.as_basic_value_enum().get_type(),
                        *ptr,
                        &format!("load_{}", self.identifier),
                    );

                    let result = match (current_value, ass_value) {
                        (BasicValueEnum::FloatValue(current_value), Value::Float(ass_value)) => {
                            builder.build_float_add(current_value, ass_value, "add")
                        }
                        _ => unreachable!(
                            "Cannot yet add-assign {:?} with {:?}",
                            current_value, ass_value
                        ),
                    };

                    builder.build_store(*ptr, result);
                }
                AssignmentType::Sub => {
                    let (ty, ptr) = get_variable(variables, &self.identifier);
                    let current_value =
                        builder.build_load(ty.as_basic_value_enum().get_type(), *ptr, "load");

                    let result = match (current_value, ass_value) {
                        (BasicValueEnum::FloatValue(current_value), Value::Float(ass_value)) => {
                            builder.build_float_sub(current_value, ass_value, "sub")
                        }
                        _ => unreachable!(
                            "Cannot yet sub-assign {:?} with {:?}",
                            current_value, ass_value
                        ),
                    };

                    builder.build_store(*ptr, result);
                }
                AssignmentType::Mul => {
                    let (ty, ptr) = get_variable(variables, &self.identifier);
                    let current_value = builder.build_load(
                        ty.as_basic_value_enum().get_type(),
                        *ptr,
                        &format!("load_{}", self.identifier),
                    );

                    let result = match (current_value, ass_value) {
                        (BasicValueEnum::FloatValue(current_value), Value::Float(ass_value)) => {
                            builder.build_float_mul(current_value, ass_value, "mul")
                        }
                        _ => unreachable!(
                            "Cannot yet mul-assign {:?} with {:?}",
                            current_value, ass_value
                        ),
                    };

                    builder.build_store(*ptr, result);
                }
                AssignmentType::Div => {
                    let (ty, ptr) = get_variable(variables, &self.identifier);
                    let current_value = builder.build_load(
                        ty.as_basic_value_enum().get_type(),
                        *ptr,
                        &format!("load_{}", self.identifier),
                    );

                    let result = match (current_value, ass_value) {
                        (BasicValueEnum::FloatValue(current_value), Value::Float(ass_value)) => {
                            builder.build_float_div(current_value, ass_value, "add")
                        }
                        _ => unreachable!(
                            "Cannot yet div-assign {:?} with {:?}",
                            current_value, ass_value
                        ),
                    };

                    builder.build_store(*ptr, result);
                }
                AssignmentType::Pow => {
                    todo!()
                }
            };
        }

        Value::Void
    }
}
