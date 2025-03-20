use crate::compiler::{Scanner, Token, ErrorReporter};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    error_reporter: &'a mut dyn ErrorReporter,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, error_reporter: &'a mut dyn ErrorReporter) -> Self {
        Self {
            tokens,
            current: 0,
            error_reporter,
        }
    }

    pub fn parse(&mut self) -> Result<(), ()> {
        // Parser implementation will go here
        Ok(())
    }
} 