#![no_std]
#![no_main]
#![allow(non_snake_case)]

extern "C" { fn println_n(x: f32); }

#[no_mangle]
pub extern "C" fn stdlibTest() -> f32 {
    unsafe { println_n(2.8) };
    
    5.5
}
