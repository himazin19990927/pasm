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