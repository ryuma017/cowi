use std::path::PathBuf;

use clap::Parser;

use cowi::{interpreter::Interpreter, lexer::Lexer};

#[derive(Parser)]
#[clap(about, version, author, long_about = None)]
struct Args {
    /// Path to COW file
    #[clap(parse(from_os_str))]
    file_path: PathBuf,

    /// Specify log filter level
    #[clap(short, long = "log-level", value_parser)]
    log_level: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let arg = Args::parse();

    if let Some(log_level) = arg.log_level {
        std::env::set_var("RUST_LOG", log_level);
    }
    env_logger::init();

    let lexer = Lexer::new(arg.file_path)?;
    let program = lexer.lex()?;
    let interpreter = Interpreter::new(program);

    if let Err(e) = interpreter.run() {
        log::error!("{e}.");
    } else {
        log::info!("Done.\n")
    }

    Ok(())
}
