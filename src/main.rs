mod opcode;
pub mod stack;
mod interpreter;
mod error;

use std::array;

use opcode::Opcode;
use interpreter::Interpreter;

fn main() {
    let a: u8 = 101;

    let arr: [u8; 7] = [0x1, 0x1, 0x2 , 0x76, 0x93, 0x93, 0xa9];
    let script = opcode::parse_script(&arr).unwrap();

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
