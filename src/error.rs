use std::fmt;

/// Errors raised while parsing or executing a script.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptError {
    /// An opcode needed more stack items than were available.
    StackUnderflow,
    /// `OP_EQUALVERIFY` compared two different values.
    EqualVerifyFailed,
    /// The script ended with an empty stack or a false top item.
    EvalFalse,
    /// A push opcode declared more bytes than the script contains.
    TruncatedPush,
    /// A byte that does not map to any opcode this interpreter parses.
    UnknownOpcode(u8),
    /// A recognised opcode whose semantics are not implemented.
    UnimplementedOpcode(&'static str),
    /// A numeric operand longer than the 4-byte consensus limit.
    NumberOverflow,
    /// A numeric operand that is not minimally encoded.
    NonMinimalNumber,
}

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptError::StackUnderflow => write!(f, "stack underflow"),
            ScriptError::EqualVerifyFailed => write!(f, "OP_EQUALVERIFY failed"),
            ScriptError::EvalFalse => write!(f, "script evaluated to false"),
            ScriptError::TruncatedPush => write!(f, "push exceeds script length"),
            ScriptError::UnknownOpcode(byte) => write!(f, "unknown opcode 0x{byte:02x}"),
            ScriptError::UnimplementedOpcode(name) => write!(f, "{name} is not implemented"),
            ScriptError::NumberOverflow => write!(f, "numeric operand exceeds 4 bytes"),
            ScriptError::NonMinimalNumber => write!(f, "numeric operand is not minimally encoded"),
        }
    }
}

impl std::error::Error for ScriptError {}
