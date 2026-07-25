use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ScriptError {
    StackUnderflow,
    EqualVerifyFailed,
    UnimplementedOpcode(String),
}

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScriptError::StackUnderflow => write!(f, "stack underflow: not enough items on the stack"),
            ScriptError::EqualVerifyFailed => write!(f, "OP_EQUALVERIFY failed: values were not equal"),
            ScriptError::UnimplementedOpcode(name) => write!(f, "opcode not implemented: {}", name),
        }
    }
}

impl std::error::Error for ScriptError {}