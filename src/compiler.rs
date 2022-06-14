use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use crate::lexer::Token;

pub struct Compiler {
    tokens: Vec<Token>,
    rx: Receiver<Token>,
    binary: Vec<u8>,
    labels: HashMap<String, i32>,
    pc: i32,
    pos: i32,
    debug: bool,
}