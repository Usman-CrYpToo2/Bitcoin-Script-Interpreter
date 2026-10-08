//! Opcode definitions and script parsing.

use crate::error::ScriptError;
use crate::num;

/// A parsed script instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opcode {
    /// Pushes the contained bytes. Produced by direct pushes (`0x01..=0x4b`),
    /// `OP_PUSHDATA1/2/4`, `OP_0`, `OP_1NEGATE` and `OP_1`..`OP_16`.
    Push(Vec<u8>),
    /// `OP_DUP` (`0x76`)
    Dup,
    /// `OP_EQUAL` (`0x87`)
    Equal,
    /// `OP_EQUALVERIFY` (`0x88`)
    EqualVerify,
    /// `OP_ADD` (`0x93`)
    Add,
    /// `OP_HASH160` (`0xa9`)
    Hash160,
    /// `OP_CHECKSIG` (`0xac`)
    CheckSig,
}

pub const OP_0: u8 = 0x00;
pub const OP_PUSHDATA1: u8 = 0x4c;
pub const OP_PUSHDATA2: u8 = 0x4d;
pub const OP_PUSHDATA4: u8 = 0x4e;
pub const OP_1NEGATE: u8 = 0x4f;
pub const OP_1: u8 = 0x51;
pub const OP_16: u8 = 0x60;
pub const OP_DUP: u8 = 0x76;
pub const OP_EQUAL: u8 = 0x87;
pub const OP_EQUALVERIFY: u8 = 0x88;
pub const OP_ADD: u8 = 0x93;
pub const OP_HASH160: u8 = 0xa9;
pub const OP_CHECKSIG: u8 = 0xac;

/// Parses serialized script bytes into opcodes.
pub fn parse_script(bytes: &[u8]) -> Result<Vec<Opcode>, ScriptError> {
    let mut ops = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let byte = bytes[i];
        i += 1;

        let op = match byte {
            OP_0 => Opcode::Push(Vec::new()),
            0x01..=0x4b => Opcode::Push(take(bytes, &mut i, byte as usize)?),
            OP_PUSHDATA1 => {
                let len = read_len(bytes, &mut i, 1)?;
                Opcode::Push(take(bytes, &mut i, len)?)
            }
            OP_PUSHDATA2 => {
                let len = read_len(bytes, &mut i, 2)?;
                Opcode::Push(take(bytes, &mut i, len)?)
            }
            OP_PUSHDATA4 => {
                let len = read_len(bytes, &mut i, 4)?;
                Opcode::Push(take(bytes, &mut i, len)?)
            }
            OP_1NEGATE => Opcode::Push(num::encode(-1)),
            OP_1..=OP_16 => Opcode::Push(num::encode(i64::from(byte - OP_1 + 1))),
            OP_DUP => Opcode::Dup,
            OP_EQUAL => Opcode::Equal,
            OP_EQUALVERIFY => Opcode::EqualVerify,
            OP_ADD => Opcode::Add,
            OP_HASH160 => Opcode::Hash160,
            OP_CHECKSIG => Opcode::CheckSig,
            other => return Err(ScriptError::UnknownOpcode(other)),
        };
        ops.push(op);
    }

    Ok(ops)
}

/// Reads a little-endian length prefix of `width` bytes.
fn read_len(bytes: &[u8], i: &mut usize, width: usize) -> Result<usize, ScriptError> {
    let prefix = take(bytes, i, width)?;
    Ok(prefix
        .iter()
        .enumerate()
        .fold(0usize, |acc, (k, &b)| acc | (usize::from(b) << (8 * k))))
}

/// Takes `len` bytes starting at `*i` and advances the cursor.
fn take(bytes: &[u8], i: &mut usize, len: usize) -> Result<Vec<u8>, ScriptError> {
    let end = i.checked_add(len).ok_or(ScriptError::TruncatedPush)?;
    let data = bytes
        .get(*i..end)
        .ok_or(ScriptError::TruncatedPush)?
        .to_vec();
    *i = end;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_small_integers() {
        let ops = parse_script(&[OP_0, OP_1NEGATE, OP_1, OP_16]).unwrap();
        assert_eq!(
            ops,
            vec![
                Opcode::Push(vec![]),
                Opcode::Push(vec![0x81]),
                Opcode::Push(vec![0x01]),
                Opcode::Push(vec![0x10]),
            ]
        );
    }

    #[test]
    fn parses_direct_push() {
        assert_eq!(
            parse_script(&[0x02, 0xab, 0xcd]).unwrap(),
            vec![Opcode::Push(vec![0xab, 0xcd])]
        );
    }

    #[test]
    fn parses_pushdata_variants() {
        let data = vec![0x42; 300];

        let mut s1 = vec![OP_PUSHDATA1, 3];
        s1.extend([1, 2, 3]);
        assert_eq!(
            parse_script(&s1).unwrap(),
            vec![Opcode::Push(vec![1, 2, 3])]
        );

        let mut s2 = vec![OP_PUSHDATA2, 0x2c, 0x01]; // 300, little-endian
        s2.extend(&data);
        assert_eq!(parse_script(&s2).unwrap(), vec![Opcode::Push(data.clone())]);

        let mut s4 = vec![OP_PUSHDATA4, 0x2c, 0x01, 0x00, 0x00];
        s4.extend(&data);
        assert_eq!(parse_script(&s4).unwrap(), vec![Opcode::Push(data)]);
    }

    #[test]
    fn rejects_truncated_pushes() {
        assert_eq!(
            parse_script(&[0x03, 0x01, 0x02]),
            Err(ScriptError::TruncatedPush)
        );
        assert_eq!(
            parse_script(&[OP_PUSHDATA1]),
            Err(ScriptError::TruncatedPush)
        );
        assert_eq!(
            parse_script(&[OP_PUSHDATA2, 0x05]),
            Err(ScriptError::TruncatedPush)
        );
        assert_eq!(
            parse_script(&[OP_PUSHDATA4, 0xff, 0xff, 0xff, 0xff]),
            Err(ScriptError::TruncatedPush)
        );
    }

    #[test]
    fn rejects_unknown_opcodes() {
        assert_eq!(parse_script(&[0x61]), Err(ScriptError::UnknownOpcode(0x61))); // OP_NOP
    }

    #[test]
    fn parses_p2pkh_template() {
        let mut script = vec![OP_DUP, OP_HASH160, 0x14];
        script.extend([0u8; 20]);
        script.extend([OP_EQUALVERIFY, OP_CHECKSIG]);
        assert_eq!(
            parse_script(&script).unwrap(),
            vec![
                Opcode::Dup,
                Opcode::Hash160,
                Opcode::Push(vec![0; 20]),
                Opcode::EqualVerify,
                Opcode::CheckSig,
            ]
        );
    }
}
