use crate::asm::print_disassembled;

mod opcodes;
mod asm;
mod compiler;
mod lexer;

fn main() {
    println!("Hello, world!");
    if let Err(e) = print_disassembled("6000808080739caf77e5b32583fd5aee70acef5deaed67059622602b5a03f41580808073c3eba2e7e18ffa583e05fad4f2fa1f63374a0fe0602b5a03f415".to_string()){
        println!("{}", e)
    }

    if let Err(e) = print_disassembled("6000808080739caf77e5b32583fd5aee70acef5deaed67059622602b5a03f41580808073c3eba2e7e18ffa583e05fad4f2fa1f63374a0fe0602b5a03f415".to_string()){
        println!("{}", e)
    }
}
