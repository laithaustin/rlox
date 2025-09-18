use crate::compiler::lox_class::LoxClass;

#[derive(Debug)]
pub struct LoxInstance {
    pub klass: Box<LoxClass>,
}

impl LoxInstance {
    pub fn new(klass: Box<LoxClass>) -> Self {
        Self { klass }
    }
}
