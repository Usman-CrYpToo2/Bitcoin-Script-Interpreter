use crate::opcode::Opcode;
use crate::stack::Stack;
use crate::error::ScriptError;

#[derive(PartialEq)]
pub struct Interpreter {
    stack: Stack,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { stack: Stack::new() }
    }

    pub fn run(&mut self, script: &[Opcode]) -> Result<(), ScriptError> {
        for op in script {
            self.execute(op)?;
        }
        Ok(())
    }

    fn execute(&mut self, op: &Opcode) -> Result<(), ScriptError> {
        match op {
            Opcode::Push(value) => {
                self.stack.push(*value);
                Ok(())
            }
            Opcode::Add => {
                let b = self.stack.pop().ok_or(ScriptError::StackUnderflow)?;
                let a = self.stack.pop().ok_or(ScriptError::StackUnderflow)?;
                self.stack.push(a + b);
                Ok(())
            }
            Opcode::Dup => {
                let top = self.stack.pop().ok_or(ScriptError::StackUnderflow)?;
                self.stack.push(top);
                self.stack.push(top);
                Ok(())
            }
            Opcode::Hash160 => {
                Err(ScriptError::UnimplementedOpcode("HASH160".to_string()))
            }
            Opcode::EqualVerify => {
                let b = self.stack.pop().ok_or(ScriptError::StackUnderflow)?;
                let a = self.stack.pop().ok_or(ScriptError::StackUnderflow)?;
                if a != b {
                    return Err(ScriptError::EqualVerifyFailed);
                }
                Ok(())
            }
            Opcode::CheckSig => {
                self.stack.push(1);
                Ok(())
            }
        }
    }

    pub fn stack_len(&self) -> usize {
        self.stack.len()
    }

    pub fn print_stack(&self) {
        print!("{:?}", self.stack.items.clone());
    }

}


#[cfg(test)]
mod test {
    use crate::opcode::Opcode::Push;

use super::*;


     #[test]
     fn add_two_number() {
         let mut intetpreter = Interpreter::new();
         let opcode = &[
             Opcode::Push(100),
             Opcode::Push(100),
             Opcode::Add
         ];
         let res =intetpreter.run(opcode);
         intetpreter.print_stack();
         assert_eq!(res, Ok(()));
         assert_eq!(*intetpreter.stack.items.last().unwrap(), 200);

     }
    

     #[test]
     fn wrong_addition_of_empty_stack() {
         let mut interpreter = Interpreter::new();
         let op = &[
             Opcode::Add
         ];

        let res =  interpreter.run(op);
        
        assert_eq!(res, Err(ScriptError::StackUnderflow));
     }

      #[test]
      fn dup_opcode_test() {
        let mut interpreter = Interpreter::new();
        let op = &[
            Push(100),
            Opcode::Dup
        ];

        let res = interpreter.run(op);
        assert_eq!(res, Ok(()));

        assert_eq!(*interpreter.stack.items.get(0).unwrap(), 100);
        assert_eq!(*interpreter.stack.items.get(1).unwrap(), 100);
     
      }
       
       #[test]
      fn equalverify_opcode_true() {
        let mut interpreter = Interpreter::new();
        let op = &[
            Push(100),
            Push(100),
            Opcode::EqualVerify
        ];

        let res = interpreter.run(op);
        assert_eq!(res, Ok(()));
      }

      #[test]
      fn equalverify_opcodes_fails() {
        let mut interpreter = Interpreter::new();
        let op = &[
            Push(100),
            Push(102),
            Opcode::EqualVerify
        ]; 


        let res = interpreter.run(op);
        assert_eq!(res, Err(ScriptError::EqualVerifyFailed));
      }

}