// defines what an opcode IS (using an enum)

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Push(i64), 
    Add,
    Dup,
    Hash160,
    EqualVerify,
    CheckSig
}