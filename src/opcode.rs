// defines what an opcode IS (using an enum)

use std::{io::Bytes, vec};

use crate::error::ScriptError;

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Push(Vec<u8>), 
    Add,
    Dup,
    Hash160,
    EqualVerify,
    CheckSig
}


pub fn parse_script(bytes: &[u8]) -> Result<Vec<Opcode>, ScriptError> {
    let mut ops = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let byte = bytes[i];
        match byte {
            0x01..=0x4b => {
                let len = byte as usize;
                if i + 1 + len > bytes.len() {
                    return Err(ScriptError::UnimplementedOpcode("truncated push".to_string()));
                }
                let data = bytes[i + 1..i + 1 + len].to_vec();
                ops.push(Opcode::Push(data));
                i += 1 + len;
            }
            0x76 => { ops.push(Opcode::Dup); i += 1; }
            0xa9 => { ops.push(Opcode::Hash160); i += 1; }
            0x88 => { ops.push(Opcode::EqualVerify); i += 1; }
            0xac => { ops.push(Opcode::CheckSig); i += 1; }
            0x93 => { ops.push(Opcode::Add); i += 1}
            other => {
                return Err(ScriptError::UnimplementedOpcode(format!("0x{:02x}", other)));
            }
        }
    }

    Ok(ops)
}