#[derive(Debug, PartialEq, Clone)]
pub struct Code {
    code: u16,
    ty: Type,
}

impl Code {
    pub fn new(code: u16, ty: Type) -> Self {
        Code { code: code, ty: ty }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    I,
    R,
}
