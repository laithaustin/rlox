use std::fmt;

#[derive(Debug)]
pub struct LoxClass {
    pub name: String,
}

impl LoxClass {
    fn new(name: String) -> Self {
        LoxClass { name }
    }
}
