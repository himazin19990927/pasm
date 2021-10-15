use std::num::ParseIntError;



#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    Int(LitInt),
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
