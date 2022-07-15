use std::io::{self, Read, Write};

use anyhow::{bail, ensure, Result};

use crate::{errors::ErrorKind, instruction::Instruction};

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

    pub fn run(mut self) -> Result<()> {
        loop {
            if self.cursor >= self.program.len() {
                log::info!("Completed successfully.");
                break Ok(());
            }

            match self.program[self.cursor] {
                Instruction::LoopEnd => self.loop_end()?,
                Instruction::DecrementPointer => self.decrement_pointer()?,
                Instruction::IncrementPointer => self.increment_pointer()?,
                Instruction::ExecuteValue => self.execute_value()?,
                Instruction::ReadOrWrite => self.read_or_write()?,
                Instruction::DecrementByte => self.decrement_byte()?,
                Instruction::IncrementByte => self.increment_byte()?,
                Instruction::LoopBigin => self.loop_begin()?,
                Instruction::SetZero => self.set_zero()?,
                Instruction::CopyOrPaste => self.copy_or_paste()?,
                Instruction::WriteStdout => self.write_stdout()?,
                Instruction::ReadStdin => self.read_stdin()?,
            }
            log::debug!(
                "\n\tmemory value: {:?}\n\tpointer: {}\n\tcursor: {}\n\tmemory state: {:?}",
                self.memory[self.pointer],
                self.pointer,
                self.cursor,
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
        log::info!("mOo: decrement pointer");
        Ok(())
    }

    /// moO
    fn increment_pointer(&mut self) -> Result<()> {
        if self.pointer == MEMORY_SIZE {
            bail!(ErrorKind::OverFlow);
        }
        self.pointer += 1;
        log::info!("moO: increment pointer");
        Ok(())
    }

    /// mOO
    fn execute_value(&self) -> Result<()> {
        unimplemented!()
    }

    // TODO: input_stream からの入力を受け付ける
    /// Moo
    fn read_or_write(&mut self) -> Result<()> {
        let current_memory = &mut self.memory[self.pointer];
        if *current_memory == 0 {
            log::info!(
                "Moo: current memory block has 0 - reading a single ASCII charactor from STDIN."
            );
            let mut buf = [0; 1];
            io::stdin().read_exact(&mut buf).unwrap();
            ensure!(buf.is_ascii(), ErrorKind::InvalidValue);
            *current_memory = buf[0] as i32;
        } else {
            log::info!("Moo: current memory block is not 0 - print the ASCII character that corresponds to the value in the current memory block to STDOUT.");
            io::stdout().lock().write_all(&[*current_memory as u8]).unwrap();
        }
        Ok(())
    }

    /// MOo
    fn decrement_byte(&mut self) -> Result<()> {
        // wrapping_add: オーバーフローを無視して減算する
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
        log::info!("MOo: decrement current memory value by 1");
        Ok(())
    }

    /// MoO
    fn increment_byte(&mut self) -> Result<()> {
        // wrapping_sub: オーバーフローを無視して加算する
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
        log::info!("MoO: increment current memory value by 1");
        Ok(())
    }

    /// MOO
    fn loop_begin(&mut self) -> Result<()> {
        unimplemented!()
    }

    /// OOO
    fn set_zero(&mut self) -> Result<()> {
        unimplemented!()
    }

    /// MMM
    fn copy_or_paste(&mut self) -> Result<()> {
        unimplemented!()
    }

    /// OOM
    fn write_stdout(&mut self) -> Result<()> {
        unimplemented!()
    }

    /// oom
    fn read_stdin(&mut self) -> Result<()> {
        unimplemented!()
    }
}
