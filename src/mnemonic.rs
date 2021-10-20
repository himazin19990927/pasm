use crate::register::Register;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Label(String),
    Mnemonic(Mnemonic),
}

impl Item {
    pub fn label(label: String) -> Self {
        Self::Label(label)
    }

    pub fn instr_r(funct: FunctR, dst: Register, src: Register) -> Self {
        Self::Mnemonic(Mnemonic::instr_r(funct, dst, src))
    }

    pub fn instr_i(opcode: OpcodeI, dst: Register, immediate: i8) -> Self {
        Self::Mnemonic(Mnemonic::instr_i(opcode, dst, immediate))
    }

    pub fn instr_b(opcode: OpcodeB, src: Register, label: String) -> Self {
        Self::Mnemonic(Mnemonic::instr_b(opcode, src, label))
    }

    pub fn instr_j(opcode: OpcodeJ, label: String) -> Self {
        Self::Mnemonic(Mnemonic::instr_j(opcode, label))
    }

    pub fn instr_jr(funct: FunctJR, dst: Register) -> Self {
        Self::Mnemonic(Mnemonic::instr_jr(funct, dst))
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Item::Label(l) => write!(f, ":{}", l),
            Item::Mnemonic(m) => write!(f, "{}", m),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Mnemonic {
    R(InstructionR),
    I(InstructionI),
    B(InstructionB),
    J(InstructionJ),
    JR(InstructionJR),
}

impl Mnemonic {
    pub fn instr_r(funct: FunctR, dst: Register, src: Register) -> Self {
        Mnemonic::R(InstructionR { funct, dst, src })
    }

    pub fn instr_i(opcode: OpcodeI, dst: Register, immediate: i8) -> Self {
        Mnemonic::I(InstructionI {
            opcode,
            dst,
            immediate,
        })
    }

    pub fn instr_b(opcode: OpcodeB, src: Register, label: String) -> Self {
        Mnemonic::B(InstructionB { opcode, src, label })
    }

    pub fn instr_j(opcode: OpcodeJ, label: String) -> Self {
        Mnemonic::J(InstructionJ { opcode, label })
    }

    pub fn instr_jr(funct: FunctJR, dst: Register) -> Self {
        Mnemonic::JR(InstructionJR { funct, dst })
    }
}

impl Display for Mnemonic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Mnemonic::R(i) => write!(f, "{}", i),
            Mnemonic::I(i) => write!(f, "{}", i),
            Mnemonic::B(i) => write!(f, "{}", i),
            Mnemonic::J(i) => write!(f, "{}", i),
            Mnemonic::JR(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InstructionR {
    pub funct: FunctR,
    pub dst: Register,
    pub src: Register,
}

impl Display for InstructionR {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.funct {
            FunctR::LD => write!(f, "{} {}, ({})", self.funct, self.dst, self.src),
            FunctR::ST => write!(f, "{} {}, ({})", self.funct, self.src, self.dst),
            _ => write!(f, "{} {}, {}", self.funct, self.dst, self.src),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FunctR {
    NOP,
    MV,
    AND,
    OR,
    SL,
    SR,
    ADD,
    SUB,
    ST,
    LD,
}

impl FunctR {
    pub fn id(&self) -> u16 {
        match &self {
            FunctR::NOP => 0b00000,
            FunctR::MV => 0b00001,
            FunctR::AND => 0b00010,
            FunctR::OR => 0b00011,
            FunctR::SL => 0b00100,
            FunctR::SR => 0b00101,
            FunctR::ADD => 0b00110,
            FunctR::SUB => 0b00111,
            FunctR::ST => 0b01000,
            FunctR::LD => 0b01001,
        }
    }
}

impl Display for FunctR {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FunctR::NOP => todo!(),
            FunctR::MV => todo!(),
            FunctR::AND => todo!(),
            FunctR::OR => todo!(),
            FunctR::SL => todo!(),
            FunctR::SR => todo!(),
            FunctR::ADD => write!(f, "ADD"),
            FunctR::SUB => todo!(),
            FunctR::ST => write!(f, "ST"),
            FunctR::LD => write!(f, "LD"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InstructionI {
    pub opcode: OpcodeI,
    pub dst: Register,
    pub immediate: i8,
}

impl Display for InstructionI {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}, #{}", self.opcode, self.dst, self.immediate)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OpcodeI {
    LDI,
    LDIU,
    ADDI,
    ADDIU,
    LDHI,
}

impl OpcodeI {
    pub fn id(&self) -> u16 {
        match &self {
            OpcodeI::LDI => 0b01000,
            OpcodeI::LDIU => 0b01001,
            OpcodeI::ADDI => 0b01100,
            OpcodeI::ADDIU => 0b01101,
            OpcodeI::LDHI => 0b01010,
        }
    }
}

impl Display for OpcodeI {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OpcodeI::LDI => write!(f, "LDI"),
            OpcodeI::LDIU => todo!(),
            OpcodeI::ADDI => write!(f, "ADDI"),
            OpcodeI::ADDIU => todo!(),
            OpcodeI::LDHI => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InstructionB {
    pub opcode: OpcodeB,
    pub src: Register,
    pub label: String,
}

impl Display for InstructionB {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}, {}", self.opcode, self.src, self.label)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OpcodeB {
    BEZ,
    BNZ,
    BPL,
    BMI,
}

impl OpcodeB {
    pub fn id(&self) -> u16 {
        match &self {
            OpcodeB::BEZ => 0b10000,
            OpcodeB::BNZ => todo!(),
            OpcodeB::BPL => todo!(),
            OpcodeB::BMI => todo!(),
        }
    }
}

impl Display for OpcodeB {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OpcodeB::BEZ => write!(f, "BEZ"),
            OpcodeB::BNZ => todo!(),
            OpcodeB::BPL => todo!(),
            OpcodeB::BMI => todo!(),
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
pub enum OpcodeJ {
    JMP,
    JAL,
}

impl OpcodeJ {
    pub fn id(&self) -> u16 {
        match &self {
            OpcodeJ::JMP => 0b10100,
            OpcodeJ::JAL => 0b10101,
        }
    }
}

impl Display for OpcodeJ {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OpcodeJ::JMP => write!(f, "JMP"),
            &OpcodeJ::JAL => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InstructionJR {
    pub funct: FunctJR,
    pub dst: Register,
}

impl Display for InstructionJR {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.funct, self.dst)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FunctJR {
    JR,
    JALR,
}

impl FunctJR {
    pub fn id(&self) -> u16 {
        match &self {
            FunctJR::JR => 0b01010,
            FunctJR::JALR => 0b11000,
        }
    }
}

impl Display for FunctJR {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FunctJR::JR => write!(f, "JR"),
            FunctJR::JALR => todo!(),
        }
    }
}
