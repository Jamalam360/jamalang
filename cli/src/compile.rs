use std::{fs, path::PathBuf, time::Instant};

use jamalang::parser::parse_ast;
use jamalang::compiler::Compiler;
use inkwell::context::Context;

use crate::CompileTarget;

pub fn cmd(
    path: String,
    target: CompileTarget,
    destination: Option<String>,
    time_compilation: bool,
) -> anyhow::Result<()> {
    let script = fs::read_to_string(path.clone())?;

    let file = parse_ast(&path, &script)?;

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    let mut compiler = Compiler::new(&context, &module, &builder);

    if !time_compilation {
        compiler.compile(file);
    } else {
        let start = Instant::now();
        compiler.compile(file);
        println!("Time: {:?}", start.elapsed());
    }

    let mut destination = destination.map_or_else(|| PathBuf::from(path), PathBuf::from);

    match target {
        CompileTarget::Binary => {
            if destination.as_os_str() == "stdout" {
                panic!("Cannot write binary to stdout")
            }

            destination.set_extension(".bitcode");
            compiler.module.write_bitcode_to_path(&destination);
        }
        CompileTarget::IR => {
            if destination.as_os_str() == "stdout" {
                compiler.module.print_to_stderr();
            } else {
                destination.set_extension("ll");
                compiler
                    .module
                    .print_to_file(&destination)
                    .map_err(|s| anyhow::anyhow!(s.to_string()))?;
            }
        }
    }

    Ok(())
}
