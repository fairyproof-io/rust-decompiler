use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Copy, Eq,PartialOrd, PartialEq, Hash, TryFromPrimitive)]
pub enum OpCode {
    // 0x0 range - arithmetic ops.
    STOP = 0x0,
    ADD = 0x1,
    MUL = 0x2,
    SUB = 0x3,
    DIV = 0x4,
    SDIV = 0x5,
    MOD = 0x6,
    SMOD = 0x7,
    ADDMOD = 0x8,
    MULMOD = 0x9,
    EXP = 0xa,
    SIGNEXTEND = 0xb,
    // 0x10 range - comparison ops.
    LT = 0x10,
    GT = 0x11,
    SLT = 0x12,
    SGT = 0x13,
    EQ = 0x14,
    ISZERO = 0x15,
    AND = 0x16,
    OR = 0x17,
    XOR = 0x18,
    NOT = 0x19,
    BYTE = 0x1a,
    SHL = 0x1b,
    SHR = 0x1c,
    SAR = 0x1d,
    // 0x20 range - crypto.
    KECCAK256 = 0x20,
    // 0x30 range - closure state.
    ADDRESS = 0x30,
    BALANCE = 0x31,
    ORIGIN = 0x32,
    CALLER = 0x33,
    CALLVALUE = 0x34,
    CALLDATALOAD = 0x35,
    CALLDATASIZE = 0x36,
    CALLDATACOPY = 0x37,
    CODESIZE = 0x38,
    CODECOPY = 0x39,
    GASPRICE = 0x3a,
    EXTCODESIZE = 0x3b,
    EXTCODECOPY = 0x3c,
    RETURNDATASIZE = 0x3d,
    RETURNDATACOPY = 0x3e,
    EXTCODEHASH = 0x3f,
    // 0x40 range - block operations.
    BLOCKHASH = 0x40,
    COINBASE = 0x41,
    TIMESTAMP = 0x42,
    NUMBER = 0x43,
    DIFFICULTY = 0x44,
    GASLIMIT = 0x45,
    CHAINID = 0x46,
    SELFBALANCE = 0x47,
    BASEFEE = 0x48,
    // 0x50 range - 'storage' and execution.
    POP = 0x50,
    MLOAD = 0x51,
    MSTORE = 0x52,
    MSTORE8 = 0x53,
    SLOAD = 0x54,
    SSTORE = 0x55,
    JUMP = 0x56,
    JUMPI = 0x57,
    PC = 0x58,
    MSIZE = 0x59,
    GAS = 0x5a,
    JUMPDEST = 0x5b,
    PUSH0 = 0x5f,
    // 0x60 range - pushes.
    PUSH1 = 0x60,
    PUSH2,
    PUSH3,
    PUSH4,
    PUSH5,
    PUSH6,
    PUSH7,
    PUSH8,
    PUSH9,
    PUSH10,
    PUSH11,
    PUSH12,
    PUSH13,
    PUSH14,
    PUSH15,
    PUSH16,
    PUSH17,
    PUSH18,
    PUSH19,
    PUSH20,
    PUSH21,
    PUSH22,
    PUSH23,
    PUSH24,
    PUSH25,
    PUSH26,
    PUSH27,
    PUSH28,
    PUSH29,
    PUSH30,
    PUSH31,
    PUSH32,
    // 0x80 range - dups.
    DUP1 = 0x80,
    DUP2,
    DUP3,
    DUP4,
    DUP5,
    DUP6,
    DUP7,
    DUP8,
    DUP9,
    DUP10,
    DUP11,
    DUP12,
    DUP13,
    DUP14,
    DUP15,
    DUP16,
    // 0x90 range - swaps.
    SWAP1 = 0x90,
    SWAP2,
    SWAP3,
    SWAP4,
    SWAP5,
    SWAP6,
    SWAP7,
    SWAP8,
    SWAP9,
    SWAP10,
    SWAP11,
    SWAP12,
    SWAP13,
    SWAP14,
    SWAP15,
    SWAP16,
    // 0xa0 range - logging ops.
    LOG0 = 0xa0,
    LOG1,
    LOG2,
    LOG3,
    LOG4,
    // 0xf0 range - closures.
    CREATE = 0xf0,
    CALL = 0xf1,
    CALLCODE = 0xf2,
    RETURN = 0xf3,
    DELEGATECALL = 0xf4,
    CREATE2 = 0xf5,

    STATICCALL = 0xfa,
    REVERT = 0xfd,
    INVALID = 0xfe,
    SELFDESTRUCT = 0xff,
}

// Since the opcodes aren't all in order we can't use a regular slice.
use lazy_static::lazy_static;
lazy_static!(
    static ref OPCODE_MAP: HashMap<OpCode, &'static str> = vec![
    // 0x0 range - arithmetic ops.
    (OpCode::STOP, "STOP"),
    (OpCode::ADD, "ADD"),
    (OpCode::MUL, "MUL"),
    (OpCode::SUB, "SUB"),
    (OpCode::DIV, "DIV"),
    (OpCode::SDIV, "SDIV"),
    (OpCode::MOD, "MOD"),
    (OpCode::SMOD, "SMOD"),
    (OpCode::EXP, "EXP"),
    (OpCode::NOT, "NOT"),
    (OpCode::LT, "LT"),
    (OpCode::GT, "GT"),
    (OpCode::SLT, "SLT"),
    (OpCode::SGT, "SGT"),
    (OpCode::EQ, "EQ"),
    (OpCode::ISZERO, "ISZERO"),
    (OpCode::SIGNEXTEND, "SIGNEXTEND"),
    // 0x10 range - bit ops.
    (OpCode::AND, "AND"),
    (OpCode::OR, "OR"),
    (OpCode::XOR, "XOR"),
    (OpCode::BYTE, "BYTE"),
    (OpCode::SHL, "SHL"),
    (OpCode::SHR, "SHR"),
    (OpCode::SAR, "SAR"),
    (OpCode::ADDMOD, "ADDMOD"),
    (OpCode::MULMOD, "MULMOD"),
    // 0x20 range - crypto.
    (OpCode::KECCAK256, "KECCAK256"),
    // 0x30 range - closure state.
    (OpCode::ADDRESS, "ADDRESS"),
    (OpCode::BALANCE, "BALANCE"),
    (OpCode::ORIGIN, "ORIGIN"),
    (OpCode::CALLER, "CALLER"),
    (OpCode::CALLVALUE, "CALLVALUE"),
    (OpCode::CALLDATALOAD, "CALLDATALOAD"),
    (OpCode::CALLDATASIZE, "CALLDATASIZE"),
    (OpCode::CALLDATACOPY, "CALLDATACOPY"),
    (OpCode::CODESIZE, "CODESIZE"),
    (OpCode::CODECOPY, "CODECOPY"),
    (OpCode::GASPRICE, "GASPRICE"),
    (OpCode::EXTCODESIZE, "EXTCODESIZE"),
    (OpCode::EXTCODECOPY, "EXTCODECOPY"),
    (OpCode::RETURNDATASIZE, "RETURNDATASIZE"),
    (OpCode::RETURNDATACOPY, "RETURNDATACOPY"),
    (OpCode::EXTCODEHASH, "EXTCODEHASH"),
    // 0x40 range - block operations.
    (OpCode::BLOCKHASH, "BLOCKHASH"),
    (OpCode::COINBASE, "COINBASE"),
    (OpCode::TIMESTAMP, "TIMESTAMP"),
    (OpCode::NUMBER, "NUMBER"),
    (OpCode::DIFFICULTY, "DIFFICULTY"),
    (OpCode::GASLIMIT, "GASLIMIT"),
    (OpCode::CHAINID, "CHAINID"),
    (OpCode::SELFBALANCE, "SELFBALANCE"),
    (OpCode::BASEFEE, "BASEFEE"),
    // 0x50 range - 'storage' and execution.
    (OpCode::POP, "POP"),
    //DUP,    "DUP",
    //SWAP,   "SWAP",
    (OpCode::MLOAD, "MLOAD"),
    (OpCode::MSTORE, "MSTORE"),
    (OpCode::MSTORE8, "MSTORE8"),
    (OpCode::SLOAD, "SLOAD"),
    (OpCode::SSTORE, "SSTORE"),
    (OpCode::JUMP, "JUMP"),
    (OpCode::JUMPI, "JUMPI"),
    (OpCode::PC, "PC"),
    (OpCode::MSIZE, "MSIZE"),
    (OpCode::GAS, "GAS"),
    (OpCode::JUMPDEST, "JUMPDEST"),
    (OpCode::PUSH0, "PUSH0"),
    // 0x60 range - push.
    (OpCode::PUSH1, "PUSH1"),
    (OpCode::PUSH2, "PUSH2"),
    (OpCode::PUSH3, "PUSH3"),
    (OpCode::PUSH4, "PUSH4"),
    (OpCode::PUSH5, "PUSH5"),
    (OpCode::PUSH6, "PUSH6"),
    (OpCode::PUSH7, "PUSH7"),
    (OpCode::PUSH8, "PUSH8"),
    (OpCode::PUSH9, "PUSH9"),
    (OpCode::PUSH10, "PUSH10"),
    (OpCode::PUSH11, "PUSH11"),
    (OpCode::PUSH12, "PUSH12"),
    (OpCode::PUSH13, "PUSH13"),
    (OpCode::PUSH14, "PUSH14"),
    (OpCode::PUSH15, "PUSH15"),
    (OpCode::PUSH16, "PUSH16"),
    (OpCode::PUSH17, "PUSH17"),
    (OpCode::PUSH18, "PUSH18"),
    (OpCode::PUSH19, "PUSH19"),
    (OpCode::PUSH20, "PUSH20"),
    (OpCode::PUSH21, "PUSH21"),
    (OpCode::PUSH22, "PUSH22"),
    (OpCode::PUSH23, "PUSH23"),
    (OpCode::PUSH24, "PUSH24"),
    (OpCode::PUSH25, "PUSH25"),
    (OpCode::PUSH26, "PUSH26"),
    (OpCode::PUSH27, "PUSH27"),
    (OpCode::PUSH28, "PUSH28"),
    (OpCode::PUSH29, "PUSH29"),
    (OpCode::PUSH30, "PUSH30"),
    (OpCode::PUSH31, "PUSH31"),
    (OpCode::PUSH32, "PUSH32"),
    (OpCode::DUP1, "DUP1"),
    (OpCode::DUP2, "DUP2"),
    (OpCode::DUP3, "DUP3"),
    (OpCode::DUP4, "DUP4"),
    (OpCode::DUP5, "DUP5"),
    (OpCode::DUP6, "DUP6"),
    (OpCode::DUP7, "DUP7"),
    (OpCode::DUP8, "DUP8"),
    (OpCode::DUP9, "DUP9"),
    (OpCode::DUP10, "DUP10"),
    (OpCode::DUP11, "DUP11"),
    (OpCode::DUP12, "DUP12"),
    (OpCode::DUP13, "DUP13"),
    (OpCode::DUP14, "DUP14"),
    (OpCode::DUP15, "DUP15"),
    (OpCode::DUP16, "DUP16"),
    (OpCode::SWAP1, "SWAP1"),
    (OpCode::SWAP2, "SWAP2"),
    (OpCode::SWAP3, "SWAP3"),
    (OpCode::SWAP4, "SWAP4"),
    (OpCode::SWAP5, "SWAP5"),
    (OpCode::SWAP6, "SWAP6"),
    (OpCode::SWAP7, "SWAP7"),
    (OpCode::SWAP8, "SWAP8"),
    (OpCode::SWAP9, "SWAP9"),
    (OpCode::SWAP10, "SWAP10"),
    (OpCode::SWAP11, "SWAP11"),
    (OpCode::SWAP12, "SWAP12"),
    (OpCode::SWAP13, "SWAP13"),
    (OpCode::SWAP14, "SWAP14"),
    (OpCode::SWAP15, "SWAP15"),
    (OpCode::SWAP16, "SWAP16"),
    (OpCode::LOG0, "LOG0"),
    (OpCode::LOG1, "LOG1"),
    (OpCode::LOG2, "LOG2"),
    (OpCode::LOG3, "LOG3"),
    (OpCode::LOG4, "LOG4"),
    // 0xf0 range.
    (OpCode::CREATE, "CREATE"),
    (OpCode::CALL, "CALL"),
    (OpCode::RETURN, "RETURN"),
    (OpCode::CALLCODE, "CALLCODE"),
    (OpCode::DELEGATECALL, "DELEGATECALL"),
    (OpCode::CREATE2, "CREATE2"),
    (OpCode::STATICCALL, "STATICCALL"),
    (OpCode::REVERT, "REVERT"),
    (OpCode::INVALID, "INVALID"),
    (OpCode::SELFDESTRUCT, "SELFDESTRUCT"),
].into_iter().collect();
);


impl OpCode {
    // IsPush specifies if an opcode is a PUSH opcode.
    pub fn is_push(self) -> bool {
        match self {
            Self::PUSH1
            | Self::PUSH2
            | Self::PUSH3
            | Self::PUSH4
            | Self::PUSH5
            | Self::PUSH6
            | Self::PUSH7
            | Self::PUSH8
            | Self::PUSH9
            | Self::PUSH10
            | Self::PUSH11
            | Self::PUSH12
            | Self::PUSH13
            | Self::PUSH14
            | Self::PUSH15
            | Self::PUSH16
            | Self::PUSH17
            | Self::PUSH18
            | Self::PUSH19
            | Self::PUSH20
            | Self::PUSH21
            | Self::PUSH22
            | Self::PUSH23
            | Self::PUSH24
            | Self::PUSH25
            | Self::PUSH26
            | Self::PUSH27
            | Self::PUSH28
            | Self::PUSH29
            | Self::PUSH30
            | Self::PUSH31
            | Self::PUSH32 => true,
            _ => false,
        }
    }
}

impl Clone for OpCode {
    fn clone(&self) -> Self {
        *self
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // todo: catch a error
        write!(f, "{}", *OPCODE_MAP.get(self).unwrap())
    }
}
