# Self-review: Bitcoin Script Interpreter

| | |
|---|---|
| **Scope** | `src/` at commit `2724fcf` |
| **Reference** | Bitcoin Core `script/interpreter.cpp`, `script/script.h` |
| **Date** | 2026-10 |

## Summary

| ID | Title | Severity | Status |
|---|---|---|---|
| BSI-01 | `OP_CHECKSIG` accepts every input | Critical | Fixed |
| BSI-02 | `OP_ADD` uses big-endian unsigned numbers | High | Fixed |
| BSI-03 | Final stack is never evaluated | High | Fixed |
| BSI-04 | Standard push opcodes are rejected | Medium | Fixed |
| BSI-05 | Truncated push reported as an unimplemented opcode | Low | Fixed |

Severity reflects the impact if the interpreter were used to validate spends.

---

## BSI-01: `OP_CHECKSIG` accepts every input

**Severity:** Critical

```rust
Opcode::CheckSig => {
    self.stack.push(vec![1]);
    Ok(())
}
```

`OP_CHECKSIG` pushed `true` without popping the signature and public key or
verifying anything. Any P2PKH output whose public-key hash matched would be
spendable with an arbitrary signature. The stack was also left with two extra
items, so scripts after it ran against the wrong state.

**Fix.** `OP_CHECKSIG` now returns `ScriptError::UnimplementedOpcode` until
signature verification exists. Failing closed is the only safe default for an
authentication opcode. Covered by `p2pkh_hash_check_passes_and_checksig_fails_closed`.

---

## BSI-02: `OP_ADD` uses big-endian unsigned numbers

**Severity:** High

```rust
fn bytes_decimal(bytes: &[u8]) -> u64 {
    let mut num = 0u64;
    for b in bytes { num = (num << 8) | *b as u64; }
    num
}
// ...
self.stack.push(addition.to_be_bytes().to_vec());
```

Script numbers are little-endian sign-magnitude with minimal encoding
(`CScriptNum`). The original code read operands big-endian and unsigned, and
wrote results as 8 big-endian bytes:

| Input | Bitcoin | Original |
|---|---|---|
| `[0x81]` | -1 | 129 |
| `[0x00, 0x01]` | 256 | 1 |
| `2 + 3` result | `[0x05]` | `[0,0,0,0,0,0,0,0x05]` |

Any comparison with a correctly encoded number failed, and operands over 8
bytes overflowed the shift silently.

**Fix.** [`num.rs`](../src/num.rs) implements `CScriptNum` encoding and
decoding, including the 4-byte operand limit and the minimal-encoding check.
Covered by the Bitcoin Core `scriptnum_tests` vectors in `num::tests`.

---

## BSI-03: Final stack is never evaluated

**Severity:** High

`run` returned `Ok(())` whenever no opcode errored. A script ending in `[]`,
`[0x00]`, `[0x80]` or an empty stack was reported as a success, whereas
`VerifyScript` rejects all of them.

**Fix.** `Interpreter::verify` requires a non-empty stack whose top item passes
`CastToBool`. `run` keeps `EvalScript` semantics for callers that inspect the
stack. Covered by `empty_and_negative_zero_stacks_fail` and `equal_pushes_canonical_false`.

---

## BSI-04: Standard push opcodes are rejected

**Severity:** Medium

The parser only handled direct pushes `0x01` to `0x4b`. `OP_0`, `OP_1NEGATE`,
`OP_1` to `OP_16` and `OP_PUSHDATA1/2/4` were reported as unimplemented, so
most real scripts could not be parsed.

**Fix.** All push opcodes are parsed, with little-endian length prefixes for
`OP_PUSHDATA*`. Covered by `opcode::tests`.

---

## BSI-05: Truncated push reported as an unimplemented opcode

**Severity:** Low

A push whose length ran past the end of the script returned
`UnimplementedOpcode("truncated push")`, which misreports a malformed script
as a missing feature.

**Fix.** A dedicated `ScriptError::TruncatedPush`, with checked arithmetic on
the cursor. Covered by `rejects_truncated_pushes`.
