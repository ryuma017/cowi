use crate::instruction::Instruction;

const MEMORY_SIZE: usize = 30000;

pub struct Interpreter {
    program: Vec<Instruction>,
    memory: [u8; MEMORY_SIZE],
    pointer: usize,
    position: usize,
    register: Option<i32>,
}

impl Interpreter {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            memory: [0; MEMORY_SIZE],
            pointer: 0,
            position: 0,
            register: None,
        }
    }

    fn run(mut self) -> Result<(), std::io::Error> {
        loop {
            if self.position < self.program.len() {
                log::info!("Completed successfully.");
                break Ok(());
            }

            match self.program[self.position] {
                Instruction::LoopEnd => todo!(),
                Instruction::DecrementPointer => todo!(),
                Instruction::IncrementPointer => todo!(),
                Instruction::ExecuteValue => todo!(),
                Instruction::ReadOrWrite => todo!(),
                Instruction::DecrementByte => todo!(),
                Instruction::IncrementByte => todo!(),
                Instruction::LoopBigin => todo!(),
                Instruction::ReadStdin => todo!(),
                Instruction::SetZero => todo!(),
                Instruction::CopyOrPaste => todo!(),
                Instruction::WriteStdout => todo!(),
            }
        }
    }
}
