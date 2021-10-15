use crate::{lit::*, register::*};

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    LD { dst: Register, src: Parenthesized },
    LDI { dst: Register, src: Lit },
    ST { dst: Parenthesized, src: Register },
    ADD { dst: Register, src: Register },
    ADDI { dst: Register, src: Lit },
}
