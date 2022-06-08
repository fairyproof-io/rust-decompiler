use crate::opcodes::OpCode;
use hex::FromHexError;
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use std::error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum ASMError {
    IncompletePushInstruction(u64),
    HexDecodeError(FromHexError),
    OpCodeNotExist(TryFromPrimitiveError<OpCode>),
}

impl Display for ASMError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncompletePushInstruction(pc) => {
                write!(f, "incomplete push instruction at {}", pc)
            }
            Self::HexDecodeError(err) => {
                write!(f, "{}", err.to_string())
            }
            Self::OpCodeNotExist(err) => {
                write!(f, "opcode {} does not exist", err.number)
            }
        }
    }
}

impl error::Error for ASMError {}

impl From<FromHexError> for ASMError {
    fn from(err: FromHexError) -> Self {
        Self::HexDecodeError(err)
    }
}

impl From<TryFromPrimitiveError<OpCode>> for ASMError {
    fn from(err: TryFromPrimitiveError<OpCode>) -> Self {
        Self::OpCodeNotExist(err)
    }
}

// Iterator for disassembled EVM instructions
struct InstructionIterator<'a> {
    code: &'a [u8],
    pc: u64,
    arg: &'a [u8],
    op: OpCode,
    error: Option<ASMError>,
    started: bool,
}

impl<'a> InstructionIterator<'a> {
    fn new(code: &'a [u8]) -> Self {
        Self {
            code,
            pc: 0,
            arg: "".as_bytes(),
            op: OpCode::STOP,
            error: None,
            started: false,
        }
    }
}

impl<'a> Iterator for InstructionIterator<'a> {
    type Item = (u64, OpCode, &'a [u8]);

    // next returns item if there is a next instruction and moves on.
    fn next(&mut self) -> Option<Self::Item> {
        if self.code.len() as u64 <= self.pc {
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
        match OpCode::try_from_primitive(self.code[self.pc as usize]) {
            Ok(op) => self.op = op,
            Err(e) => {
                self.error = Some(ASMError::OpCodeNotExist(e));
                //println!("{:?}", self.error.as_ref().unwrap().to_string());
                return None;
            }
        }
        if self.op.is_push() {
            let a = self.op as u64 - OpCode::PUSH1 as u64 + 1;
            let u = self.pc + 1 + a;
            if self.code.len() as u64 <= self.pc || (self.code.len() as u64) < u {
                self.error = Some(ASMError::IncompletePushInstruction(self.pc));
                //println!("{:?}", self.error.as_ref().unwrap().to_string());
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
pub fn print_disassembled(code: String) -> Result<(), ASMError> {
    let bytecode = hex::decode(code)?;
    let mut it = InstructionIterator::new(bytecode.as_slice());
    for (pc, op, arg) in it.by_ref() {
        if arg.len() != 0 && 0 < arg.len() {
            println!("{:#08x}:  {} 0x{}", pc, op, hex::encode(arg));
        } else {
            println!("{:#08x}:  {}", pc, op);
        }
    }

    it.error.map_or(Ok(()), |err| Err(err))
}

// disassemble returns all disassembled EVM instructions in human-readable format.
//pub fn disassemble(script: &[u8]) -> Result<String, ASMError> {
//  Ok("".to_string())
//}
