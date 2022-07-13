use std::{path::PathBuf, fs::File, io::{Read, Cursor}};

use anyhow::Context;
use clap::Parser;

use cowi::instruction::AsInstruction;

#[derive(Parser)]
#[clap(about, version, author, long_about = None)]
struct Arg {
    /// Path to COW file
    #[clap(parse(from_os_str))]
    file_path: PathBuf
}

fn main() -> anyhow::Result<()> {
    let arg = Arg::parse();
    let bytes: Vec<u8> = {
        let mut bytes = vec![];
        let mut file = File::open(&arg.file_path)
            .with_context(|| format!("Failed to open `{}`", arg.file_path.display()))?;
        file.read_to_end(&mut bytes)
            .context("Failed to read bytes from source.")?;
        bytes
    };
    let mut buffer = [0; 3];
    let length = bytes.len();
    let mut cursor = Cursor::new(bytes);
    let mut program = vec![];
    loop {
        if cursor.position() == (length - 2) as u64 {
            break;
        }
        cursor.read_exact(&mut buffer)?;
        if let Some(i) = buffer.as_instruction() {
            program.push(i);
        }
        let current_pos = cursor.position();
        cursor.set_position(current_pos - 2);
    };
    dbg!(program);
    Ok(())
}