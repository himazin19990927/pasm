#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// A literal token: `1`
    Lit(Lit),

    /// The `LD` token.
    LD,

    /// The `ST` token.
    ST,

    /// The `ADD` token.
    ADD,

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

    /// The `,` token.
    Comma,

    /// The `#` token.
    Sharp,

    /// The `(` token
    OpenParen,

    /// The `)` token
    CloseParen,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    Int(LitInt),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LitInt {
    pub digits: String,
}
