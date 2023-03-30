use std::{fs, time::Instant};

use jamalang::parser::parse_ast;

pub fn cmd(path: String, time_parsing: bool) -> anyhow::Result<()> {
    let script = fs::read_to_string(path.clone())?;

    let file = if !time_parsing {
        parse_ast(&path, &script)?
    } else {
        let start = Instant::now();
        let file = parse_ast(&path, &script)?;
        println!("Time: {:?}", start.elapsed());
        file
    };

    println!("{:#?}", file.statements);
    Ok(())
}
