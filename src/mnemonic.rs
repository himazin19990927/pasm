use crate::register::Register;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Mnemonic {
    Label(String),
    I(InstructionI),
    R(InstructionR),
}

impl Display for Mnemonic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Mnemonic::Label(l) => write!(f, ":{}", l),
            Mnemonic::I(i) => write!(f, "{}", i),
            Mnemonic::R(i) => write!(f, "{}", i),
        }
    }
}

impl From<InstructionI> for Mnemonic {
    fn from(i: InstructionI) -> Self {
        Mnemonic::I(i)
    }
}

impl From<InstructionR> for Mnemonic {
    fn from(i: InstructionR) -> Self {
        Mnemonic::R(i)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InstructionI {
    pub opcode: Opcode,
    pub dst: Register,
    pub immediate: i8,
}

impl Display for InstructionI {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}, #{}", self.opcode, self.dst, self.immediate)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InstructionR {
    pub funct: Funct,
    pub dst: Register,
    pub src: Register,
}

impl Display for InstructionR {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.funct {
            Funct::LD => write!(f, "{} {}, ({})", self.funct, self.dst, self.src),
            Funct::ST => write!(f, "{} {}, ({})", self.funct, self.src, self.dst),
            _ => write!(f, "{} {}, {}", self.funct, self.dst, self.src),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Opcode {
    LDI,
    ADDI,
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::LDI => write!(f, "LDI"),
            Opcode::ADDI => write!(f, "ADDI"),
        }
    }
}

impl Opcode {
    pub fn id(&self) -> u16 {
        match &self {
            Opcode::LDI => 0b01000,
            Opcode::ADDI => 0b01100,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Funct {
    LD,
    ST,
    ADD,
}

impl Display for Funct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Funct::LD => write!(f, "LD"),
            Funct::ST => write!(f, "ST"),
            Funct::ADD => write!(f, "ADD"),
        }
    }
}

impl Funct {
    pub fn id(&self) -> u16 {
        match &self {
            Funct::LD => 0b01001,
            Funct::ST => 0b01000,
            Funct::ADD => 0b00110,
        }
    }
}
