#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// A number token: `1`
    Num(String),

    /// A ident token: `label0`, `end`
    Ident(String),

    /// The `NOP` token.
    NOP,

    /// The `MV` token.
    MV,

    /// The `AND` token.
    AND,

    /// The `OR` token.
    OR,

    /// The `SL` token.
    SL,

    /// The `SR` token.
    SR,

    /// The `ADD` token.
    ADD,

    /// The `SUB` token.
    SUB,

    /// The `ST` token.
    ST,

    /// The `LD` token.
    LD,

    /// The `LDI` token.
    LDI,

    /// The `LDIU` token.
    LDIU,

    /// The `ADDI` token.
    ADDI,

    /// The `ADDIU` token.
    ADDIU,

    /// The `LDHI` token.
    LDHI,

    /// The `BEZ` token.
    BEZ,

    /// The `BNZ` token.
    BNZ,

    /// The `BPL` token.
    BPL,

    /// The `BMI` token.
    BMI,

    /// The `JMP` token.
    JMP,

    /// The `JAL` token.
    JAL,

    /// The `JR` token.
    JR,

    /// The `JALR` token.
    JALR,

    /// The `R0` token.
    R0,

    /// The `R1` token.
    R1,

    /// The `R2` token.
    R2,

    /// The `R3` token.
    R3,

    /// The `R4` token.
    R4,

    /// The `R5` token.
    R5,

    /// The `R6` token.
    R6,

    /// The `R7` token.
    R7,

    /// The `#` token.
    Sharp,

    /// The `-` token.
    Minus,

    /// The `,` token.
    Comma,

    /// The `:` token.
    Colon,

    /// The `(` token
    OpenParen,

    /// The `)` token
    CloseParen,
}
