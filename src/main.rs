use std::path::PathBuf;

use clap::Parser;

use cowi::{interpreter::Interpreter, lexer::Lexer};

#[derive(Parser)]
#[clap(about, version, author, long_about = None)]
struct Arg {
    /// Path to COW file
    #[clap(parse(from_os_str))]
    file_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let arg = Arg::parse();

    let lexer = Lexer::new(arg.file_path)?;
    let program = lexer.lex()?;
    let interpreter = Interpreter::new(program);

    if let Err(e) = interpreter.run() {
        log::error!("{e}");
    }

    Ok(())
}
