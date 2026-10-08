//! End-to-end tests: serialized script bytes in, verification result out.

use bitcoin_script_interpreter::interpreter::hash160;
use bitcoin_script_interpreter::{Interpreter, ScriptError, parse_script};

fn hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

fn verify(script: &[u8]) -> (Result<(), ScriptError>, Vec<Vec<u8>>) {
    let ops = parse_script(script).expect("script parses");
    let mut interpreter = Interpreter::new();
    let result = interpreter.verify(&ops);
    (result, interpreter.stack().to_vec())
}

#[test]
fn add_then_equal_succeeds() {
    // OP_2 OP_3 OP_ADD OP_5 OP_EQUAL
    let (result, stack) = verify(&hex("5253935587"));
    assert_eq!(result, Ok(()));
    assert_eq!(stack, vec![vec![0x01]]);
}

#[test]
fn add_result_is_minimally_encoded() {
    // OP_2 OP_3 OP_ADD leaves [0x05], not an 8-byte big-endian value.
    let (_, stack) = verify(&hex("525393"));
    assert_eq!(stack, vec![vec![0x05]]);
}

#[test]
fn add_handles_signed_and_multibyte_operands() {
    // 0x81 is -1: OP_1NEGATE OP_1 OP_ADD == 0 (empty vector), which is false.
    let (result, stack) = verify(&hex("4f5193"));
    assert_eq!(stack, vec![Vec::<u8>::new()]);
    assert_eq!(result, Err(ScriptError::EvalFalse));

    // [0xff, 0x00] (255) + OP_1 == [0x00, 0x01] (256), little-endian.
    let (result, stack) = verify(&hex("02ff005193"));
    assert_eq!(result, Ok(()));
    assert_eq!(stack, vec![vec![0x00, 0x01]]);
}

#[test]
fn add_rejects_non_minimal_and_oversized_operands() {
    // [0x05, 0x00] is a non-minimal encoding of 5.
    let (result, _) = verify(&hex("0205005193"));
    assert_eq!(result, Err(ScriptError::NonMinimalNumber));

    // A 5-byte operand exceeds the consensus limit: <0102030405> OP_1 OP_ADD.
    let (result, _) = verify(&hex("0501020304055193"));
    assert_eq!(result, Err(ScriptError::NumberOverflow));
}

#[test]
fn equal_pushes_canonical_false() {
    // OP_1 OP_2 OP_EQUAL leaves the empty vector and fails verification.
    let (result, stack) = verify(&hex("515287"));
    assert_eq!(stack, vec![Vec::<u8>::new()]);
    assert_eq!(result, Err(ScriptError::EvalFalse));
}

#[test]
fn empty_and_negative_zero_stacks_fail() {
    assert_eq!(verify(&[]).0, Err(ScriptError::EvalFalse));
    assert_eq!(verify(&hex("0180")).0, Err(ScriptError::EvalFalse)); // push [0x80]
}

#[test]
fn dup_underflows_on_empty_stack() {
    assert_eq!(verify(&hex("76")).0, Err(ScriptError::StackUnderflow));
}

#[test]
fn hash160_matches_known_vectors() {
    assert_eq!(
        hash160(&[]),
        hex("b472a266d0bd89c13706a4132ccfb16f7c3b9fcb")
    );

    // Compressed secp256k1 generator point; its address is
    // 1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH.
    let generator = hex("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
    assert_eq!(
        hash160(&generator),
        hex("751e76e8199196d454941c45d1b3a323f1433bd6")
    );
}

/// scriptSig `<pubkey>` followed by a P2PKH scriptPubKey.
fn p2pkh(pubkey_hash: &str) -> Vec<u8> {
    let mut script = vec![0x21];
    script.extend(hex(
        "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
    ));
    script.extend([0x76, 0xa9, 0x14]);
    script.extend(hex(pubkey_hash));
    script.extend([0x88, 0xac]);
    script
}

#[test]
fn p2pkh_hash_check_passes_and_checksig_fails_closed() {
    let (result, _) = verify(&p2pkh("751e76e8199196d454941c45d1b3a323f1433bd6"));
    assert_eq!(result, Err(ScriptError::UnimplementedOpcode("OP_CHECKSIG")));
}

#[test]
fn p2pkh_rejects_wrong_pubkey_hash() {
    let (result, _) = verify(&p2pkh("0000000000000000000000000000000000000000"));
    assert_eq!(result, Err(ScriptError::EqualVerifyFailed));
}
