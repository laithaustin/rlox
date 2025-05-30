use crate::compiler::error::LoxError;
use crate::compiler::expr::Object;

pub enum ControlFlow {
    None,
    Return(Object),
    // can add more here as we go
}

pub type FlowResult<T> = Result<(T, ControlFlow), LoxError>;

// Helper for simple return case - a little wrapper on OK that allows for control flow handling
pub fn ok<T>(value: T) -> FlowResult<T> {
    Ok((value, ControlFlow::None))
}

pub fn return_value(value: Object) -> FlowResult<Object> {
    Ok((Object::Nil, ControlFlow::Return(value)))
}

pub fn extract_value<T>(flow_res: FlowResult<T>) -> Result<T, LoxError> {
    flow_res.map(|(value, _)| value)
}
