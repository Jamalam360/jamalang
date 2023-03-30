use std::ffi::{c_void, CStr};

use inkwell::{context::Context, types::FunctionType};

#[no_mangle]
pub extern "C" fn builtin_println_number(x: f32) {
    println!("{}", x);
}

#[no_mangle]
pub extern "C" fn builtin_println_char(x: i8) {
    println!("{}", (x as u8) as char);
}

#[no_mangle]
pub extern "C" fn builtin_println_bool(x: bool) {
    println!("{}", x);
}

#[derive(Debug)]
pub struct Builtin<'a> {
    pub name: String,
    pub c_name: *const i8,
    pub function: *mut c_void,
    pub f_type: FunctionType<'a>,
}

macro_rules! cstr {
    ($s:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($s, "\0").as_bytes()).as_ptr()
    };
}

pub fn builtins<'a>(context: &'a Context) -> Vec<Builtin<'a>> {
    unsafe {
        vec![
            Builtin {
                name: "println_n".to_string(),
                c_name: cstr!("builtin_println_number"),
                function: builtin_println_number as *mut c_void,
                f_type: context
                    .void_type()
                    .fn_type(&[context.f32_type().into()], false),
            },
            Builtin {
                name: "println_c".to_string(),
                c_name: cstr!("builtin_println_char"),
                function: builtin_println_char as *mut c_void,
                f_type: context
                    .void_type()
                    .fn_type(&[context.i8_type().into()], false),
            },
            Builtin {
                name: "println_b".to_string(),
                c_name: cstr!("builtin_println_bool"),
                function: builtin_println_bool as *mut c_void,
                f_type: context
                    .void_type()
                    .fn_type(&[context.bool_type().into()], false),
            },
        ]
    }
}
