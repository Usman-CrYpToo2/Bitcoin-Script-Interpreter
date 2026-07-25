mod opcode;
pub mod stack;
mod interpreter;
mod error;

use opcode::Opcode;
use interpreter::Interpreter;

fn main() {
    let script = vec![
        Opcode::Push(5),
        Opcode::Push(3),
        Opcode::Add,
        Opcode::Push(5),
        Opcode::Add,
    ];

    let mut interp = Interpreter::new();


    
    match interp.run(&script) {
        Ok(()) => 
               {
                  println!("Script succeeded. Final stack length: {}", interp.stack_len());
                   interp.print_stack();
               },
        Err(e) => {
            interp.print_stack();
            println!("Script failed: {}", e);
        }
        
    }
    
 
}
