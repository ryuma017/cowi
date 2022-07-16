use std::io::{self, BufRead, Read, Write};

use anyhow::{bail, ensure, Result};

use crate::{
    errors::ErrorKind,
    instruction::{AsInstruction, Instruction},
};

const MEMORY_SIZE: usize = 30000;

pub struct Interpreter {
    program: Vec<Instruction>,
    memory: [i32; MEMORY_SIZE],
    pointer: usize,
    cursor: usize,
    register: Option<i32>,
    // input_stream: Option<Vec<u8>>,
}

impl Interpreter {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            memory: [0; MEMORY_SIZE],
            pointer: 0,
            cursor: 0,
            register: None,
        }
    }

    fn instruction_matches<R, W>(
        &mut self,
        instruction: Instruction,
        stdin: &mut R,
        stdout: &mut W,
    ) -> Result<()>
    where
        R: BufRead + Read,
        W: Write,
    {
        match instruction {
            Instruction::LoopEnd => self.loop_end(),
            Instruction::DecrementPointer => self.decrement_pointer(),
            Instruction::IncrementPointer => self.increment_pointer(),
            Instruction::ExecuteValue => self.execute_value(stdin, stdout),
            Instruction::ReadOrWrite => self.read_or_write(stdin, stdout),
            Instruction::DecrementByte => self.decrement_byte(),
            Instruction::IncrementByte => self.increment_byte(),
            Instruction::LoopBigin => self.loop_begin(),
            Instruction::SetZero => self.set_zero(),
            Instruction::CopyOrPaste => self.copy_or_paste(),
            Instruction::WriteStdout => self.write_stdout(stdout),
            Instruction::ReadStdin => self.read_stdin(stdin),
        }
    }

    pub fn run(mut self) -> Result<Self> {
        let mut stdin = io::stdin().lock();
        let mut stdout = io::stdout().lock();

        loop {
            if self.cursor >= self.program.len() {
                log::debug!("Completed successfully.");
                break Ok(self);
            }

            self.instruction_matches(self.program[self.cursor], &mut stdin, &mut stdout)?;

            log::debug!(
                "\n\tmemory value: {:?}\n\tpointer: {}\n\tregister: {:?}\n\tmemory state: {:?}",
                self.memory[self.pointer],
                self.pointer,
                self.register,
                &self.memory[..20]
            );
            self.cursor += 1;
        }
    }

    /// moo
    fn loop_end(&mut self) -> Result<()> {
        unimplemented!()
    }

    /// mOo
    fn decrement_pointer(&mut self) -> Result<()> {
        if self.pointer == 0 {
            bail!(ErrorKind::OverFlow);
        }
        self.pointer -= 1;
        log::debug!("mOo: decrement pointer");
        Ok(())
    }

    /// moO
    fn increment_pointer(&mut self) -> Result<()> {
        if self.pointer == MEMORY_SIZE {
            bail!(ErrorKind::OverFlow);
        }
        self.pointer += 1;
        log::debug!("moO: increment pointer");
        Ok(())
    }

    /// mOO
    fn execute_value<R: BufRead + Read, W: Write>(
        &mut self,
        stdin: &mut R,
        stdout: &mut W,
    ) -> Result<()> {
        let instruction_or_none = self.memory[self.pointer].as_instruction();
        match instruction_or_none {
            None => bail!(ErrorKind::InvalidCode),
            Some(instruction) if instruction != Instruction::ExecuteValue => {
                bail!(ErrorKind::InfiniteLoop)
            }
            Some(instruction) => self.instruction_matches(instruction, stdin, stdout),
        }
    }

    // TODO: input_stream からの入力を受け付ける
    /// Moo
    fn read_or_write<R: Read, W: Write>(&mut self, stdin: &mut R, stdout: &mut W) -> Result<()> {
        let current_memory = &mut self.memory[self.pointer];
        if *current_memory == 0 {
            log::debug!(
                "Moo: current memory block has 0 - reading a single ASCII charactor from STDIN."
            );
            let mut buf = [0; 1];
            stdin.read_exact(&mut buf).unwrap();
            ensure!(buf.is_ascii(), ErrorKind::NotAscii);
            *current_memory = buf[0] as i32;
        } else {
            log::debug!("Moo: current memory block is not 0 - writing the ASCII character that corresponds to the value in the current memory block to STDOUT.");
            stdout.write_all(&[*current_memory as u8]).unwrap();
        }
        Ok(())
    }

    /// MOo
    fn decrement_byte(&mut self) -> Result<()> {
        // wrapping_add: オーバーフローを無視して減算する
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
        log::debug!("MOo: decrement current memory value by 1");
        Ok(())
    }

    /// MoO
    fn increment_byte(&mut self) -> Result<()> {
        // wrapping_sub: オーバーフローを無視して加算する
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
        log::debug!("MoO: increment current memory value by 1");
        Ok(())
    }

    /// MOO
    fn loop_begin(&mut self) -> Result<()> {
        unimplemented!()
    }

    /// OOO
    fn set_zero(&mut self) -> Result<()> {
        self.memory[self.pointer] = 0;
        Ok(())
    }

    /// MMM
    fn copy_or_paste(&mut self) -> Result<()> {
        let current_memory = &mut self.memory[self.pointer];
        if let Some(value) = self.register {
            *current_memory = value;
            self.register = None;
        } else {
            self.register = Some(*current_memory);
        }
        Ok(())
    }

    /// OOM
    fn write_stdout<W: Write>(&mut self, stdout: &mut W) -> Result<()> {
        stdout
            .write_all(self.memory[self.pointer].to_string().as_bytes())
            .unwrap();
        log::debug!("OOM: writing value of current memory block to STDOUT as an integer");
        Ok(())
    }

    /// oom
    fn read_stdin<R: Read + BufRead>(&mut self, stdin: &mut R) -> Result<()> {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        if let Ok(integer) = buf.trim_end().parse::<i32>() {
            self.memory[self.pointer] = integer
        } else {
            bail!(ErrorKind::NotInteger)
        }
        log::debug!("oom: reading an integer from STDIN and put it into the current memory block");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction::{self, *};

    impl Default for Interpreter {
        fn default() -> Self {
            Interpreter {
                program: vec![],
                memory: [0; MEMORY_SIZE],
                pointer: 0,
                cursor: 0,
                register: None,
            }
        }
    }

    fn run_with(program: Vec<Instruction>, default_pointer: usize) -> Interpreter {
        let interpreter = Interpreter {
            program,
            pointer: default_pointer,
            ..Default::default()
        };
        interpreter.run().unwrap()
    }

    fn rerun_with(program: Vec<Instruction>, state: Interpreter) -> Interpreter {
        let interpreter = Interpreter {
            program,
            cursor: 0,
            ..state
        };
        interpreter.run().unwrap()
    }

    #[test]
    fn decrement_pointer_works() {
        // mOo mOo mOo
        let program = vec![DecrementPointer, DecrementPointer, DecrementPointer];

        let state = run_with(program, 5);

        assert_eq!(state.pointer, 2);
    }

    #[test]
    fn increment_pointer_works() {
        // moO moO moO
        let program = vec![IncrementPointer, IncrementPointer, IncrementPointer];

        let state = run_with(program, 0);

        assert_eq!(state.pointer, 3);
    }

    #[test]
    fn decrement_byte_works() {
        // MOo MOo MOo
        let program = vec![DecrementByte, DecrementByte, DecrementByte];

        let state = run_with(program, 0);

        assert_eq!(state.memory[..5], [-3, 0, 0, 0, 0]);
    }

    #[test]
    fn increment_byte_works() {
        // MoO MoO MoO
        let program = vec![IncrementByte, IncrementByte, IncrementByte];

        let state = run_with(program, 0);

        assert_eq!(state.memory[..5], [3, 0, 0, 0, 0])
    }

    #[test]
    fn set_zero_works() {
        // MoO moO MoO
        let program1 = vec![IncrementByte, IncrementPointer, IncrementByte];
        // OOO mOo OOO
        let program2 = vec![SetZero, DecrementPointer, SetZero];

        let state = run_with(program1, 0);
        assert_eq!(state.memory[..5], [1, 1, 0, 0, 0]);

        let state = rerun_with(program2, state);
        assert_eq!(state.memory[..5], [0, 0, 0, 0, 0])
    }

    #[test]
    fn copy_or_paste_works() {
        // MoO MoO MMM
        let program1 = vec![IncrementByte, IncrementByte, CopyOrPaste];
        // moO MMM
        let program2 = vec![IncrementPointer, CopyOrPaste];

        let state = run_with(program1, 0);
        assert_eq!(state.memory[..5], [2, 0, 0, 0, 0]);
        assert_eq!(state.register, Some(2));

        let state = rerun_with(program2, state);
        assert_eq!(state.memory[..5], [2, 2, 0, 0, 0]);
        assert_eq!(state.register, None);
    }
}
