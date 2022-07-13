use std::path::PathBuf;

use clap::Parser;

use cowi::lexer::Lexer;

#[derive(Parser)]
#[clap(about, version, author, long_about = None)]
struct Arg {
    /// Path to COW file
    #[clap(parse(from_os_str))]
    file_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let arg = Arg::parse();

    let lexer = Lexer::new(arg.file_path)?;
    let _program = lexer.lex()?;
    Ok(())
}
