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
    program_counter: usize,
    register: Option<i32>,
}

impl Interpreter {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            memory: [0; MEMORY_SIZE],
            pointer: 0,
            program_counter: 0,
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
            Instruction::EndLoop => self.end_loop(),
            Instruction::DecrementPointer => self.decrement_pointer(),
            Instruction::IncrementPointer => self.increment_pointer(),
            Instruction::ExecuteValue => self.execute_value(stdin, stdout),
            Instruction::ReadOrWrite => self.read_or_write(stdin, stdout),
            Instruction::DecrementByte => self.decrement_byte(),
            Instruction::IncrementByte => self.increment_byte(),
            Instruction::BeginLoop => self.begin_loop(),
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
            if self.program_counter >= self.program.len() {
                log::debug!("Completed successfully.");
                break Ok(self);
            }

            self.instruction_matches(self.program[self.program_counter], &mut stdin, &mut stdout)?;

            log::debug!(
                "\n\tmemory value: {:?}\n\tpointer: {}\n\tregister: {:?}\n\tmemory state: {:?}",
                self.memory[self.pointer],
                self.pointer,
                self.register,
                &self.memory[..20]
            );
            self.program_counter += 1;
        }
    }

    /// moo
    fn end_loop(&mut self) -> Result<()> {
        if self.memory[self.pointer] != 0 {
            ensure!(2 <= self.program_counter, ErrorKind::UnmatchedBeginLoop);
            log::debug!("moo: current memory block has {} - begin executing again starting from the found `MOO` command.", self.memory[self.pointer]);
            // pc: Program counter for this loop
            let mut pc = self.program_counter - 2;
            // c: Count `moo` and `MOO`
            let mut c = 1;
            loop {
                match self.program[pc] {
                    Instruction::BeginLoop => c -= 1,
                    Instruction::EndLoop => c += 1,
                    _ => {}
                }
                if c == 0 {
                    self.program_counter = pc;
                    break;
                }
                ensure!(0 < pc, ErrorKind::UnmatchedBeginLoop);
                pc -= 1;
            }
        } else {
            log::debug!("moo: current memory block has 0 - end loop.");
        }
        Ok(())
    }

    /// mOo
    fn decrement_pointer(&mut self) -> Result<()> {
        if self.pointer == 0 {
            bail!(ErrorKind::OverFlow);
        }
        self.pointer -= 1;
        log::debug!("mOo: decrement pointer.");
        Ok(())
    }

    /// moO
    fn increment_pointer(&mut self) -> Result<()> {
        if self.pointer == MEMORY_SIZE {
            bail!(ErrorKind::OverFlow);
        }
        self.pointer += 1;
        log::debug!("moO: increment pointer.");
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
            Some(instruction) if instruction == Instruction::ExecuteValue => {
                bail!(ErrorKind::InfiniteLoop)
            }
            Some(instruction) => {
                log::debug!("mOO: execute code {}.", self.memory[self.pointer]);
                self.instruction_matches(instruction, stdin, stdout)
            }
        }
    }

    /// Moo
    fn read_or_write<R: Read, W: Write>(&mut self, stdin: &mut R, stdout: &mut W) -> Result<()> {
        let current_memory = &mut self.memory[self.pointer];
        if *current_memory == 0 {
            log::debug!(
                "Moo: current memory block has 0 - read a single ASCII charactor from STDIN."
            );
            let mut buf = [0; 1];
            stdin.read_exact(&mut buf).unwrap();
            ensure!(buf.is_ascii(), ErrorKind::NotAscii);
            *current_memory = buf[0] as i32;
        } else {
            log::debug!("Moo: current memory block has {} - write the ASCII character that corresponds to the value in the current memory block to STDOUT.", current_memory);
            stdout.write_all(&[*current_memory as u8]).unwrap();
        }
        Ok(())
    }

    /// MOo
    fn decrement_byte(&mut self) -> Result<()> {
        // wrapping_add: オーバーフローを無視して減算する
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
        log::debug!("MOo: decrement current memory value by 1.");
        Ok(())
    }

    /// MoO
    fn increment_byte(&mut self) -> Result<()> {
        // wrapping_sub: オーバーフローを無視して加算する
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
        log::debug!("MoO: increment current memory value by 1.");
        Ok(())
    }

    /// MOO
    fn begin_loop(&mut self) -> Result<()> {
        if self.memory[self.pointer] == 0 {
            ensure!(
                self.program_counter + 2 < self.program.len(),
                ErrorKind::UnmatchedEndLoop
            );
            log::debug!("MOO: current memory block has 0 - resume execution after the next matching `moo` command.");
            // pc: Program counter for this loop
            let mut pc = self.program_counter + 2;
            // c: Count `moo` and `MOO`
            let mut c = 1;

            loop {
                match self.program[pc] {
                    Instruction::BeginLoop => c += 1,
                    Instruction::EndLoop => c -= 1,
                    _ => {}
                }
                if c == 0 {
                    self.program_counter = pc;
                    break;
                }
                ensure!(pc + 1 < self.program.len(), ErrorKind::UnmatchedEndLoop);
                pc += 1;
            }
        } else {
            log::debug!(
                "MOO: current memory block has {} - continue with next command.",
                self.memory[self.pointer]
            );
        }
        Ok(())
    }

    /// OOO
    fn set_zero(&mut self) -> Result<()> {
        self.memory[self.pointer] = 0;
        log::debug!("set 0 to current memory block.");
        Ok(())
    }

    /// MMM
    fn copy_or_paste(&mut self) -> Result<()> {
        let current_memory = &mut self.memory[self.pointer];
        if let Some(value) = self.register {
            *current_memory = value;
            self.register = None;
            log::debug!("MMM: register has {} - paste the value into the current memory block and clear the register.", current_memory);
        } else {
            self.register = Some(*current_memory);
            log::debug!("MMM: no current value in register - copy current memory block value.");
        }
        Ok(())
    }

    /// OOM
    fn write_stdout<W: Write>(&mut self, stdout: &mut W) -> Result<()> {
        stdout
            .write_all(self.memory[self.pointer].to_string().as_bytes())
            .unwrap();
        log::debug!("OOM: writing value of current memory block to STDOUT as an integer.");
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
        log::debug!("oom: reading an integer from STDIN and put it into the current memory block.");
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
                program_counter: 0,
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
            program_counter: 0,
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
