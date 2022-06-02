use crate::opcodes::OpCode;
use std::error;
use std::fmt::{Display, Formatter};
use hex::FromHexError;

#[derive(Debug, PartialEq)]
pub enum ASMError {
    IncompletePushInstruction(u64),
    HexDecodeError(FromHexError),
}

impl Display for ASMError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::IncompletePushInstruction(pc) => {
                write!(f, "incomplete push instruction at {}", pc)
            },
            Self::HexDecodeError(err) => {
                write!(f, "{}", err.to_string())
            },
        }
    }
}

impl error::Error for ASMError {

}

impl From<FromHexError> for ASMError {
    fn from(err: FromHexError) -> Self {
        Self::HexDecodeError(err)
    }
}


// Iterator for disassembled EVM instructions
struct InstructionIterator<'a> {
    code: &'a[u8],
    pc: u64,
    arg: &'a[u8],
    op: OpCode,
    error: Option<ASMError>,
    started: bool,
}

impl <'a>InstructionIterator<'a> {
    fn new(code: &'a [u8]) -> Self {
        Self{
            code,
            pc: 0,
            arg: "".as_bytes(),
            op: OpCode::STOP,
            error: None,
            started: false
        }
    }
}

impl <'a> Iterator for InstructionIterator<'a> {
    type Item = (u64, OpCode, &'a [u8]);

    // next returns true if there is a next instruction and moves on.
    fn next(&mut self) -> Option<Self::Item> {
        if self.error != None || self.code.len() as u64 <=  self.pc {
            // We previously reached an error or the end.
            return None;
        }

        if self.started {
            // Since the iteration has been already started we move to the next instruction.
            if self.arg.len() != 0 {
                self.pc += self.arg.len() as u64;
            }
            self.pc += 1;
        } else {
            // We start the iteration from the first instruction.
            self.started = true;
        }

        if self.code.len() as u64 <= self.pc {
            // We reached the end.
            return None;
        }
        // todo: catch to a convert error
        self.op = self.code[self.pc as usize].try_into().unwrap();
        if self.op.is_push() {
            let a = self.op as u64 - OpCode::PUSH1 as u64 + 1;
            let u = self.pc + 1 + a;
            if self.code.len() as u64 <= self.pc || (self.code.len() as u64) < u {
                self.error = Option::from(ASMError::IncompletePushInstruction(self.pc));
                return None;
            }
            self.arg = &self.code[(self.pc + 1) as usize..u as usize];
        } else {
            self.arg = "".as_bytes();
        }
        Some((self.pc, self.op, self.arg))
    }
}



// print_disassembled pretty-print all disassembled EVM instructions to stdout.
pub fn print_disassembled(code: String) -> Result<(), ASMError>{
    let byte_code = hex::decode(code)?;
    let byte_code= byte_code.as_slice();
    let it = InstructionIterator::new(byte_code);
    for (pc, op, arg) in it {
        if arg.len() != 0 && 0 < arg.len() {
            println!("{}:{} {}", pc, op, hex::encode(arg));
        } else {
            println!("{}:{}", pc, op);

        }
    }
    Ok(())
}

// disassemble returns all disassembled EVM instructions in human-readable format.
//pub fn disassemble(script: &[u8]) -> Result<String, ASMError> {
  //  Ok("".to_string())
//}
