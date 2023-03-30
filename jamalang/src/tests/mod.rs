macro_rules! assert_outputs {
    ($($input:expr => $expected:expr),*) => {
        $(
            let file_name = "./.test_stdout.txt";
            std::fs::write(file_name, "").expect("file should be writable");
            let guard = stdio_override::StdoutOverride::override_file(file_name).expect("stdout should be overridden");
            let script = crate::parser::parse_ast("test_file", $input).expect("syntax should be valid");

            let context = inkwell::context::Context::create();
            let module = context.create_module("main");
            let builder = context.create_builder();
        
            let mut compiler = crate::compiler::Compiler::new(&context, &module, &builder);
        
            compiler.compile(script);

            let execution_engine = compiler
                .module
                .create_jit_execution_engine(inkwell::OptimizationLevel::Aggressive)
                .expect("module should be valid");
        
            unsafe {
                type Main = unsafe extern "C" fn();
                let main: inkwell::execution_engine::JitFunction<Main> = execution_engine
                    .get_function(&format!("{}_entry", "test_file"))
                    .expect("entry function should be defined");
        
                main.call();
            }

            let contents = std::fs::read_to_string(file_name).expect("file should be readable");
            drop(guard);
            assert_eq!(contents, $expected);
        )*
    };
}

#[test]
fn print_number() {
    assert_outputs!(
        "println(1)" => "1\n"
    );
}