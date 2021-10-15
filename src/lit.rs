#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    Int(LitInt),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LitInt {
    pub digits: String,
}