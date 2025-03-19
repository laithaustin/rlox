pub trait ErrorReporter {
    fn error(&mut self, line: usize, message: &str); 
}
