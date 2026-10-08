//! Command-line entry point: `bitcoin-script-interpreter <script-hex>`.
//!
//! Parses the hex-encoded script, verifies it, and prints the final stack.

use std::process::ExitCode;

use bitcoin_script_interpreter::{Interpreter, parse_script};

/// `OP_2 OP_3 OP_ADD OP_5 OP_EQUAL`
const DEFAULT_SCRIPT: &str = "5253935587";

fn main() -> ExitCode {
    let script_hex = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_SCRIPT.to_string());

    let bytes = match decode_hex(&script_hex) {
        Some(bytes) => bytes,
        None => {
            eprintln!("error: script must be an even-length hex string");
            return ExitCode::from(2);
        }
    };

    let script = match parse_script(&bytes) {
        Ok(script) => script,
        Err(e) => {
            eprintln!("parse error: {e}");
            return ExitCode::FAILURE;
        }
    };

    let mut interpreter = Interpreter::new();
    let result = interpreter.verify(&script);

    println!("script: {script_hex}");
    println!("stack (bottom to top):");
    for item in interpreter.stack() {
        println!("  [{}]", encode_hex(item));
    }

    match result {
        Ok(()) => {
            println!("result: success");
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("result: failure ({e})");
            ExitCode::FAILURE
        }
    }
}

fn decode_hex(s: &str) -> Option<Vec<u8>> {
    let s = s.trim().trim_start_matches("0x");
    if !s.is_ascii() || s.len() % 2 != 0 {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
