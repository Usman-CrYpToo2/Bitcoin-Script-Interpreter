//! A Bitcoin Script interpreter.
//!
//! Parses serialized scripts and executes a subset of opcodes with the same
//! number encoding, boolean rules and push semantics as Bitcoin Core.

pub mod error;
pub mod interpreter;
pub mod num;
pub mod opcode;
pub mod stack;

pub use error::ScriptError;
pub use interpreter::Interpreter;
pub use opcode::{Opcode, parse_script};
