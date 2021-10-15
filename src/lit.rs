use std::num::ParseIntError;

#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    Int(LitInt),
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

impl LitInt {
    pub fn to_immediate(&self) -> Result<i8, ParseIntError> {
        self.digits.parse()
    }
}
