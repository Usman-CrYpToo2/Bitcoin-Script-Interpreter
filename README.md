# Bitcoin Script Interpreter

[![CI](https://github.com/Usman-CrYpToo2/Bitcoin-Script-Interpreter/actions/workflows/ci.yml/badge.svg)](https://github.com/Usman-CrYpToo2/Bitcoin-Script-Interpreter/actions/workflows/ci.yml)

A Bitcoin Script interpreter in Rust. It parses serialized scripts and executes
them with the same number encoding, boolean rules and push semantics as Bitcoin
Core's `interpreter.cpp`.

## Supported opcodes

| Opcode | Byte | Behaviour |
|---|---|---|
| `OP_0` | `0x00` | Push the empty vector |
| Direct push | `0x01` to `0x4b` | Push the next *n* bytes |
| `OP_PUSHDATA1/2/4` | `0x4c` to `0x4e` | Push data with a 1, 2 or 4 byte little-endian length prefix |
| `OP_1NEGATE` | `0x4f` | Push `-1` |
| `OP_1` to `OP_16` | `0x51` to `0x60` | Push 1 to 16 |
| `OP_DUP` | `0x76` | Duplicate the top item |
| `OP_EQUAL` | `0x87` | Push `[0x01]` if the top two items are byte-equal, else `[]` |
| `OP_EQUALVERIFY` | `0x88` | `OP_EQUAL`, then fail unless true |
| `OP_ADD` | `0x93` | Add two script numbers |
| `OP_HASH160` | `0xa9` | `RIPEMD160(SHA256(x))` |
| `OP_CHECKSIG` | `0xac` | Recognised; fails closed with `UnimplementedOpcode` |

Any other byte is rejected with `UnknownOpcode`.

## Semantics

| Rule | Implementation |
|---|---|
| Script numbers | Little-endian sign-magnitude, minimal encoding, `[]` is zero ([`num.rs`](src/num.rs)) |
| Arithmetic operands | At most 4 bytes and minimally encoded, otherwise `NumberOverflow` / `NonMinimalNumber` |
| Booleans | `CastToBool`: any non-zero byte is true, negative zero (`[0x80]`) is false |
| Success | `verify` requires a non-empty stack with a true top item, as `VerifyScript` does |
| Execution | `run` executes without judging the final stack, as `EvalScript` does |

## Architecture

| Module | Responsibility |
|---|---|
| [`opcode`](src/opcode.rs) | Opcode set and the byte-level parser |
| [`num`](src/num.rs) | `CScriptNum` encode/decode and `CastToBool` |
| [`stack`](src/stack.rs) | Main stack of byte vectors |
| [`interpreter`](src/interpreter.rs) | Opcode execution, `run` and `verify` |
| [`error`](src/error.rs) | `ScriptError` |

## Usage

Requires Rust 1.85 or later.

```bash
cargo run -- 5253935587   # OP_2 OP_3 OP_ADD OP_5 OP_EQUAL
```

```
script: 5253935587
stack (bottom to top):
  [01]
result: success
```

As a library:

```rust
use bitcoin_script_interpreter::{Interpreter, parse_script};

let script = parse_script(&[0x52, 0x53, 0x93, 0x55, 0x87])?;
Interpreter::new().verify(&script)?;
```

## Testing

```bash
cargo test
```

24 tests. Number encodings are checked against the vectors in Bitcoin Core's
`scriptnum_tests`, and `OP_HASH160` against known address hashes. CI runs
`cargo fmt --check`, `cargo clippy -D warnings` and `cargo test` on every push.

## Status

Not yet implemented:

- `OP_CHECKSIG` signature verification, which needs a transaction model and sighash computation
- Flow control (`OP_IF`, `OP_ELSE`, `OP_ENDIF`) and the alt stack
- Standardness rules such as `MINIMALDATA` for push encodings, and the script size and opcode count limits

## Security

A self-review of the original code found 5 correctness issues, including an
`OP_CHECKSIG` that accepted every input. All are fixed and covered by tests.
See [`audits/2026-10-self-review.md`](audits/2026-10-self-review.md).

## License

MIT, see [`LICENSE`](LICENSE).
