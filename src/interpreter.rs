//! Script execution.

use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

use crate::error::ScriptError;
use crate::num::{self, MAX_NUM_SIZE};
use crate::opcode::Opcode;
use crate::stack::Stack;

/// Executes parsed scripts against a single main stack.
#[derive(Debug, Default)]
pub struct Interpreter {
    stack: Stack,
}

impl Interpreter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Executes `script`, leaving its results on the stack.
    ///
    /// This mirrors Bitcoin Core's `EvalScript`: it fails only on execution
    /// errors and does not judge the final stack. Use [`Interpreter::verify`]
    /// for a pass/fail result.
    pub fn run(&mut self, script: &[Opcode]) -> Result<(), ScriptError> {
        script.iter().try_for_each(|op| self.execute(op))
    }

    /// Executes `script` and requires a true value on top of the final stack,
    /// as Bitcoin Core's `VerifyScript` does.
    pub fn verify(&mut self, script: &[Opcode]) -> Result<(), ScriptError> {
        self.run(script)?;
        match self.stack.top() {
            Some(top) if num::cast_to_bool(top) => Ok(()),
            _ => Err(ScriptError::EvalFalse),
        }
    }

    pub fn stack(&self) -> &[Vec<u8>] {
        self.stack.as_slice()
    }

    fn execute(&mut self, op: &Opcode) -> Result<(), ScriptError> {
        match op {
            Opcode::Push(bytes) => self.stack.push(bytes.clone()),
            Opcode::Dup => {
                let top = self
                    .stack
                    .top()
                    .ok_or(ScriptError::StackUnderflow)?
                    .to_vec();
                self.stack.push(top);
            }
            Opcode::Equal => {
                let (a, b) = self.pop_pair()?;
                self.stack.push(bool_bytes(a == b));
            }
            Opcode::EqualVerify => {
                let (a, b) = self.pop_pair()?;
                if a != b {
                    return Err(ScriptError::EqualVerifyFailed);
                }
            }
            Opcode::Add => {
                let (a, b) = self.pop_pair()?;
                let sum = num::decode(&a, MAX_NUM_SIZE)? + num::decode(&b, MAX_NUM_SIZE)?;
                self.stack.push(num::encode(sum));
            }
            Opcode::Hash160 => {
                let data = self.stack.pop()?;
                self.stack.push(hash160(&data));
            }
            Opcode::CheckSig => {
                // Signature verification needs the spending transaction and a
                // sighash implementation. Failing closed is the only safe
                // behaviour until both exist.
                return Err(ScriptError::UnimplementedOpcode("OP_CHECKSIG"));
            }
        }
        Ok(())
    }

    /// Pops two items and returns them in push order: `(second-from-top, top)`.
    fn pop_pair(&mut self) -> Result<(Vec<u8>, Vec<u8>), ScriptError> {
        if self.stack.len() < 2 {
            return Err(ScriptError::StackUnderflow);
        }
        let b = self.stack.pop()?;
        let a = self.stack.pop()?;
        Ok((a, b))
    }
}

/// `RIPEMD160(SHA256(data))`.
pub fn hash160(data: &[u8]) -> Vec<u8> {
    Ripemd160::digest(Sha256::digest(data)).to_vec()
}

/// Bitcoin's canonical booleans: `[0x01]` for true, the empty vector for false.
fn bool_bytes(value: bool) -> Vec<u8> {
    if value { vec![0x01] } else { Vec::new() }
}
