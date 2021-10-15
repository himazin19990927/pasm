use crate::{lit::*, register::Register};

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    I(InstructionI),
    R(InstructionR),
}

impl From<InstructionI> for Instruction {
    fn from(i: InstructionI) -> Self {
        Instruction::I(i)
    }
}

impl From<InstructionR> for Instruction {
    fn from(i: InstructionR) -> Self {
        Instruction::R(i)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InstructionI {
    pub opcode: Opcode,
    pub dst: Register,
    pub immediate: Lit,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InstructionR {
    pub funct: Funct,
    pub dst: Register,
    pub src: Register,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Opcode {
    LDI,
    ADDI,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Funct {
    LD,
    ST,
    ADD,
}
