use crate::register::Register;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    I(InstructionI),
    R(InstructionR),
}

#[derive(Debug, PartialEq, Clone)]
pub struct InstructionI {
    pub opcode: Opcode,
    pub dst: Register,
    pub immediate: i8,
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
