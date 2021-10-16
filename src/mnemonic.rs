use crate::register::Register;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Label(String),
    Mnemonic(Mnemonic),
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Item::Label(l) => write!(f, ":{}", l),
            Item::Mnemonic(m) => write!(f, "{}", m),
        }
    }
}

impl Item {
    pub fn label(label: String) -> Self {
        Self::Label(label)
    }

    pub fn instr_i(opcode: Opcode, dst: Register, immediate: i8) -> Self {
        Self::Mnemonic(Mnemonic::instr_i(opcode, dst, immediate))
    }

    pub fn instr_r(funct: Funct, dst: Register, src: Register) -> Self {
        Self::Mnemonic(Mnemonic::instr_r(funct, dst, src))
    }

    pub fn instr_j(opcode: OpcodeJ, label: String) -> Self {
        Self::Mnemonic(Mnemonic::J(InstructionJ { opcode, label }))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Mnemonic {
    I(InstructionI),
    R(InstructionR),
    J(InstructionJ),
}

impl Mnemonic {
    pub fn instr_i(opcode: Opcode, dst: Register, immediate: i8) -> Self {
        Mnemonic::I(InstructionI {
            opcode,
            dst,
            immediate,
        })
    }

    pub fn instr_r(funct: Funct, dst: Register, src: Register) -> Self {
        Mnemonic::R(InstructionR { funct, dst, src })
    }

    pub fn instr_j(opcode: OpcodeJ, label: String) -> Self {
        Mnemonic::J(InstructionJ { opcode, label })
    }
}

impl Display for Mnemonic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Mnemonic::I(i) => write!(f, "{}", i),
            Mnemonic::R(i) => write!(f, "{}", i),
            Mnemonic::J(i) => write!(f, "{}", i),
        }
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
pub struct InstructionJ {
    pub opcode: OpcodeJ,
    pub label: String,
}

impl Display for InstructionJ {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.opcode, self.label)
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

#[derive(Debug, PartialEq, Clone)]
pub enum OpcodeJ {
    JMP,
}

impl Display for OpcodeJ {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OpcodeJ::JMP => write!(f, "JMP"),
        }
    }
}

impl OpcodeJ {
    pub fn id(&self) -> u16 {
        match &self {
            OpcodeJ::JMP => 0b10100,
        }
    }
}
