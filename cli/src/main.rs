use clap::{Parser, Subcommand, ValueEnum};

mod ast;
mod compile;
mod run;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// View the AST of a modifier file.
    Ast {
        file: String,
        #[arg(default_value = "false", long, short)]
        time_parsing: bool,
    },
    /// Runs a Modifier file, in JIT mode.
    Run {
        /// The file to run.
        file: String,
        /// Whether to time the execution of the code, not including the compilation.
        #[arg(default_value = "false", long, short)]
        time_execution: bool,
    },
    /// Compiles a Modifier file to a binary, or to IR.
    Compile {
        /// The file to compile.
        file: String,
        /// The target to compile to.
        #[arg(default_value = "binary", long, short)]
        target: CompileTarget,
        /// The destination to compile to. Can be 'stdout' or a file path.
        #[arg(long, short)]
        destination: Option<String>,
        /// Whether to time the compilation.
        #[arg(default_value = "false", long)]
        time_compilation: bool,
    },
}

#[derive(ValueEnum, Clone)]
pub enum CompileTarget {
    Binary,
    IR,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Ast { file, time_parsing } => ast::cmd(file, time_parsing),
        Commands::Run {
            file,
            time_execution,
        } => run::cmd(file, time_execution),
        Commands::Compile {
            file,
            target,
            destination,
            time_compilation,
        } => compile::cmd(file, target, destination, time_compilation),
    }
}
