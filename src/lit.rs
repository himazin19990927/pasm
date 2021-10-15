use std::num::ParseIntError;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    Int(LitInt),
}

impl Display for Lit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Lit::Int(l) => write!(f, "{}", l),
        }
    }
}

impl From<i8> for Lit {
    fn from(n: i8) -> Self {
        Lit::Int(LitInt {
            digits: n.to_string(),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LitInt {
    pub digits: String,
}

impl Display for LitInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.digits)
    }
}

impl LitInt {
    pub fn to_immediate(&self) -> Result<i8, ParseIntError> {
        self.digits.parse()
    }
}
