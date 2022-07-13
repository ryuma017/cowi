use std::{
    fs::File,
    io::{Cursor, Read},
    path::PathBuf,
};

use crate::instruction::{AsInstruction, Instruction};

const TOKEN_BYTES: usize = 3;

type Program = Vec<Instruction>;

pub struct Lexer {
    bytes: Vec<u8>,
}

impl Lexer {
    // はたして、ファイルを読むのは Lexer の仕事なのか？まぁええわ
    pub fn new(path: PathBuf) -> Result<Self, std::io::Error> {
        log::info!("Reading bytes from {}", path.display());

        let mut bytes = vec![];
        let mut file = File::open(&path).map_err(|e| {
            log::error!("Failed to open `{}`", path.display());
            e
        })?;
        file.read_to_end(&mut bytes).map_err(|e| {
            log::error!("Failed to read bytes - cause {e}");
            e
        })?;
        Ok(Self { bytes })
    }

    pub fn lex(self) -> Result<Program, std::io::Error> {
        let mut buffer = [0; TOKEN_BYTES];
        let length = self.bytes.len();
        let mut cursor = Cursor::new(self.bytes);
        let mut program = vec![];

        loop {
            if cursor.position() == (length - 2) as u64 {
                log::info!("Lexical analysis completed successfully.");
                break;
            }
            cursor.read_exact(&mut buffer)?;
            if let Some(instruction) = buffer.as_instruction() {
                program.push(instruction)
            }
            let current_position = cursor.position();
            cursor.set_position(current_position - 2);
        }

        log::debug!("Results of lexical analysis:{:#?}", program);

        Ok(program)
    }
}
