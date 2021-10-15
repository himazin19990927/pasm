use crate::lit::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    LD { dst: Register, src: Parenthesized },
    LDI { dst: Register, src: Lit },
    ST { dst: Parenthesized, src: Register },
    ADD { dst: Register, src: Register },
    ADDI { dst: Register, src: Lit },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Parenthesized(pub Register);