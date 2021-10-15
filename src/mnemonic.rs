use crate::register::Register;

pub enum Instruction {
    I(InstructionI),
    R(InstructionR),
}

pub struct InstructionI {
    pub opcode: Opcode,
    pub dst: Register,
    pub immediate: i8,
}

pub struct InstructionR {
    pub funct: Funct,
    pub dst: Register,
    pub src: Register,
}

pub enum Opcode {
    LDI,
    ADDI,
}

pub enum Funct {
    LD,
    ST,
    ADD,
}
