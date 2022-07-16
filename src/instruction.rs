/// The commands of COW.
///
/// | Code | Instruction | Description |
/// |------|-------------|-------------|
/// | 0 | moo | This command is connected to the MOO command. When encountered during normal execution, it searches the program code in reverse looking for a matching MOO command and begins executing again starting from the found MOO command. When searching, it skips the instruction that is immediately before it (see MOO).
/// | 1 | mOo | Moves current memory position back one block.
/// | 2 | moO | Moves current memory position forward one block.
/// | 3 | mOO | Execute value in current memory block as if it were an instruction. The command executed is based on the instruction code value (for example, if the current memory block contains a 2, then the moO command is executed). An invalid command exits the running program. Value 3 is invalid as it would cause an infinite loop.
/// | 4 | Moo | If current memory block has a 0 in it, read a single ASCII character from STDIN and store it in the current memory block. If the current memory block is not 0, then print the ASCII character that corresponds to the value in the current memory block to STDOUT.
/// | 5 | MOo | Decrement current memory block value by 1.
/// | 6 | MoO | Increment current memory block value by 1.
/// | 7 | MOO | If current memory block value is 0, skip next command and resume execution **after** the next matching moo command. If current memory block value is not 0, then continue with next command. **Note that the fact that it skips the command immediately following it has interesting ramifications for where the matching moo command really is. For example, the following will match the second and not the first moo: OOO MOO moo moo**
/// | 8 | OOO | Set current memory block value to 0.
/// | 9 | MMM | If no current value in register, copy current memory block value. If there is a value in the register, then paste that value into the current memory block and clear the register.
/// | 10 | OOM | Print value of current memory block to STDOUT as an integer.
/// | 11 | oom | Read an integer from STDIN and put it into the current memory block.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    EndLoop,          // moo
    DecrementPointer, // mOo
    IncrementPointer, // moO
    ExecuteValue,     // mOO
    ReadOrWrite,      // Moo
    DecrementByte,    // MOo
    IncrementByte,    // MoO
    BeginLoop,        // MOO
    SetZero,          // OOO
    CopyOrPaste,      // MMM
    WriteStdout,      // OOM
    ReadStdin,        // oom
}

pub trait AsInstruction {
    fn as_instruction(&self) -> Option<Instruction>;
}

impl AsInstruction for [u8; 3] {
    fn as_instruction(&self) -> Option<Instruction> {
        match self {
            [0x6d, 0x6f, 0x6f] => Some(Instruction::EndLoop),
            [0x6d, 0x4f, 0x6f] => Some(Instruction::DecrementPointer),
            [0x6d, 0x6f, 0x4f] => Some(Instruction::IncrementPointer),
            [0x6d, 0x4f, 0x4f] => Some(Instruction::ExecuteValue),
            [0x4d, 0x6f, 0x6f] => Some(Instruction::ReadOrWrite),
            [0x4d, 0x4f, 0x6f] => Some(Instruction::DecrementByte),
            [0x4d, 0x6f, 0x4f] => Some(Instruction::IncrementByte),
            [0x4d, 0x4f, 0x4f] => Some(Instruction::BeginLoop),
            [0x4f, 0x4f, 0x4f] => Some(Instruction::SetZero),
            [0x4d, 0x4d, 0x4d] => Some(Instruction::CopyOrPaste),
            [0x4f, 0x4f, 0x4d] => Some(Instruction::WriteStdout),
            [0x6f, 0x6f, 0x6d] => Some(Instruction::ReadStdin),
            _ => None,
        }
    }
}

impl AsInstruction for i32 {
    fn as_instruction(&self) -> Option<Instruction> {
        match self {
            0 => Some(Instruction::EndLoop),
            1 => Some(Instruction::DecrementPointer),
            2 => Some(Instruction::IncrementPointer),
            3 => Some(Instruction::ExecuteValue),
            4 => Some(Instruction::ReadOrWrite),
            5 => Some(Instruction::DecrementByte),
            6 => Some(Instruction::IncrementByte),
            7 => Some(Instruction::BeginLoop),
            8 => Some(Instruction::SetZero),
            9 => Some(Instruction::CopyOrPaste),
            10 => Some(Instruction::WriteStdout),
            11 => Some(Instruction::ReadStdin),
            _ => None,
        }
    }
}
