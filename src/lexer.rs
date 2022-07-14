use std::{
    fs::File,
    io::{Cursor, Read},
    path::PathBuf,
};

use crate::instruction::{AsInstruction, Instruction};

const TOKEN_SIZE: usize = 3;

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
        log::debug!("{:?}", bytes);
        Ok(Self { bytes })
    }

    pub fn lex(self) -> Result<Vec<Instruction>, std::io::Error> {
        let mut buffer = [0; TOKEN_SIZE];
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

        log::debug!("Results of lexical analysis: {:#?}", program);

        Ok(program)
    }
}

#[test]
fn lex_works() {
    // M: 77, m: 109, O: 79, o: 111
    let lexer = Lexer {
        bytes: vec![
            109, 111, 111, 32, 109, 79, 111, 32, 109, 111, 79, 32, 109, 79, 79, 32, 77, 111, 111,
            32, 77, 79, 111, 32, 77, 111, 79, 32, 77, 79, 79, 32, 111, 111, 109, 32, 79, 79, 79,
            32, 77, 77, 77, 32, 79, 79, 77,
        ],
    };
    assert_eq!(
        lexer.lex().unwrap(),
        vec![
            Instruction::LoopEnd,
            Instruction::DecrementPointer,
            Instruction::IncrementPointer,
            Instruction::ExecuteValue,
            Instruction::ReadOrWrite,
            Instruction::DecrementByte,
            Instruction::IncrementByte,
            Instruction::LoopBigin,
            Instruction::ReadStdin,
            Instruction::SetZero,
            Instruction::CopyOrPaste,
            Instruction::WriteStdout
        ]
    );
}
