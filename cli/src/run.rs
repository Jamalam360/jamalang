use std::{fs, time::Instant};

use jamalang::parser::parse_ast;
use jamalang::compiler::Compiler;
use inkwell::{context::Context, execution_engine::JitFunction, OptimizationLevel};

pub fn cmd(path: String, time_execution: bool) -> anyhow::Result<()> {
    let script = fs::read_to_string(path.clone())?;

    let file = parse_ast(&path, &script)?;

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    let mut compiler = Compiler::new(&context, &module, &builder);

    compiler.compile(file);

    let execution_engine = compiler
        .module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .expect("module should be valid");

    unsafe {
        type Main = unsafe extern "C" fn();
        let main: JitFunction<Main> = execution_engine
            .get_function(&format!("{}_entry", path))
            .expect("entry function should be defined");

        if !time_execution {
            main.call();
        } else {
            let start = Instant::now();
            main.call();
            println!("Time: {:?}", start.elapsed());
        }
    }

    Ok(())
}
