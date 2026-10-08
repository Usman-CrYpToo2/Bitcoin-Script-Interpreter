//! Script number encoding, matching Bitcoin Core's `CScriptNum`.
//!
//! Numbers are little-endian, sign-magnitude: the most significant bit of the
//! last byte is the sign. Zero is the empty vector. Arithmetic operands are
//! limited to 4 bytes and must be minimally encoded; results may be longer.

use crate::error::ScriptError;

/// Maximum byte length of an arithmetic operand.
pub const MAX_NUM_SIZE: usize = 4;

/// Decodes a stack item as a script number.
///
/// Rejects operands longer than `max_size` bytes and operands with redundant
/// trailing bytes (for example `[0x00]` or `[0x01, 0x00]`).
pub fn decode(bytes: &[u8], max_size: usize) -> Result<i64, ScriptError> {
    if bytes.len() > max_size {
        return Err(ScriptError::NumberOverflow);
    }
    if let Some(&last) = bytes.last() {
        // The last byte may only be 0x00 or 0x80 when it is needed to carry
        // the sign, i.e. when the byte before it has its high bit set.
        if last & 0x7f == 0 && (bytes.len() <= 1 || bytes[bytes.len() - 2] & 0x80 == 0) {
            return Err(ScriptError::NonMinimalNumber);
        }
    } else {
        return Ok(0);
    }

    let mut result: i64 = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        result |= i64::from(byte) << (8 * i);
    }

    let sign_bit = 0x80_i64 << (8 * (bytes.len() - 1));
    if result & sign_bit != 0 {
        Ok(-(result & !sign_bit))
    } else {
        Ok(result)
    }
}

/// Encodes a value as a minimal script number.
pub fn encode(value: i64) -> Vec<u8> {
    if value == 0 {
        return Vec::new();
    }

    let negative = value < 0;
    let mut magnitude = value.unsigned_abs();
    let mut out = Vec::new();
    while magnitude > 0 {
        out.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }

    // If the high bit is already used by the magnitude, append a byte to hold
    // the sign; otherwise set the sign bit in place.
    let last = out.len() - 1;
    if out[last] & 0x80 != 0 {
        out.push(if negative { 0x80 } else { 0x00 });
    } else if negative {
        out[last] |= 0x80;
    }
    out
}

/// Interprets a stack item as a boolean, matching Bitcoin Core's `CastToBool`.
///
/// Any non-zero byte is true, except that negative zero (all zero bytes with
/// a final `0x80`) is false.
pub fn cast_to_bool(bytes: &[u8]) -> bool {
    for (i, &byte) in bytes.iter().enumerate() {
        if byte != 0 {
            return !(i == bytes.len() - 1 && byte == 0x80);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // Encodings cross-checked against Bitcoin Core's scriptnum_tests.
    const VECTORS: &[(i64, &[u8])] = &[
        (0, &[]),
        (1, &[0x01]),
        (-1, &[0x81]),
        (127, &[0x7f]),
        (-127, &[0xff]),
        (128, &[0x80, 0x00]),
        (-128, &[0x80, 0x80]),
        (255, &[0xff, 0x00]),
        (256, &[0x00, 0x01]),
        (-255, &[0xff, 0x80]),
        (32767, &[0xff, 0x7f]),
        (32768, &[0x00, 0x80, 0x00]),
        (-32768, &[0x00, 0x80, 0x80]),
        (2_147_483_647, &[0xff, 0xff, 0xff, 0x7f]),
        (-2_147_483_647, &[0xff, 0xff, 0xff, 0xff]),
    ];

    #[test]
    fn encode_matches_vectors() {
        for &(value, bytes) in VECTORS {
            assert_eq!(encode(value), bytes, "encode({value})");
        }
    }

    #[test]
    fn decode_matches_vectors() {
        for &(value, bytes) in VECTORS {
            assert_eq!(
                decode(bytes, MAX_NUM_SIZE),
                Ok(value),
                "decode({bytes:02x?})"
            );
        }
    }

    #[test]
    fn decode_rejects_non_minimal() {
        for bytes in [
            &[0x00][..],
            &[0x80],
            &[0x01, 0x00],
            &[0x01, 0x80],
            &[0x7f, 0x00],
        ] {
            assert_eq!(
                decode(bytes, MAX_NUM_SIZE),
                Err(ScriptError::NonMinimalNumber),
                "{bytes:02x?}"
            );
        }
    }

    #[test]
    fn decode_rejects_oversized_operand() {
        assert_eq!(
            decode(&[0x01, 0x02, 0x03, 0x04, 0x05], MAX_NUM_SIZE),
            Err(ScriptError::NumberOverflow)
        );
    }

    #[test]
    fn encode_supports_results_beyond_operand_range() {
        // 2^31 - 1 + 2^31 - 1 needs 5 bytes; results are not size-limited.
        let sum = encode(2 * 2_147_483_647);
        assert_eq!(sum, [0xfe, 0xff, 0xff, 0xff, 0x00]);
        assert_eq!(decode(&sum, 5), Ok(4_294_967_294));
    }

    #[test]
    fn cast_to_bool_handles_negative_zero() {
        assert!(!cast_to_bool(&[]));
        assert!(!cast_to_bool(&[0x00]));
        assert!(!cast_to_bool(&[0x00, 0x00]));
        assert!(!cast_to_bool(&[0x80]));
        assert!(!cast_to_bool(&[0x00, 0x80]));
        assert!(cast_to_bool(&[0x01]));
        assert!(cast_to_bool(&[0x80, 0x00]));
        assert!(cast_to_bool(&[0x00, 0x01]));
    }
}
