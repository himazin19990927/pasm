#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// A number token: `1`
    Num(String),

    /// A ident token: `label0`, `end`
    Ident(String),

    /// The `LD` token.
    LD,

    /// The `LDI` token.
    LDI,

    /// The `ST` token.
    ST,

    /// The `ADD` token.
    ADD,

    /// The `ADDI` token.
    ADDI,

    /// The `JMP` token.
    JMP,

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
