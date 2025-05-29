use lox::compiler::error::{ErrorReporter, LoxError};

pub struct TestErrorReporter {
    pub errors: Vec<(usize, String)>,
    pub runtime_errors: Vec<String>,
}

impl TestErrorReporter {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            runtime_errors: Vec::new(),
        }
    }

    pub fn assert_no_errors(&self) {
        assert!(self.errors.is_empty(), "Expected no errors, but got: {:?}", self.errors);
    }

    pub fn assert_errors(&self, expected_errors: &[(usize, &str)]) {
        assert_eq!(
            self.errors.len(),
            expected_errors.len(),
            "Expected {} errors, but got {}",
            expected_errors.len(),
            self.errors.len()
        );
        for (i, (line, message)) in expected_errors.iter().enumerate() {
            assert_eq!(self.errors[i].0, *line, "Error line mismatch");
            assert_eq!(self.errors[i].1, *message, "Error message mismatch");
        }
    }
    
    pub fn assert_no_runtime_errors(&self) {
        assert!(self.runtime_errors.is_empty(), "Expected no runtime errors, but got: {:?}", self.runtime_errors);
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty() || !self.runtime_errors.is_empty()
    }
}

impl ErrorReporter for TestErrorReporter {
    fn error(&mut self, line: usize, message: &str) {
        self.errors.push((line, message.to_string()));
    }
    
    fn runtime_error(&mut self, error: &LoxError) {
        self.runtime_errors.push(error.to_string());
    }
} 