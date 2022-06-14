use crate::lexer::TokenType::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::char;
use std::str::Chars;
use proc_macro::bridge::PanicMessage::String;

type stataFn = fn(&Lexer) -> stataFn;

#[repr(i32)]
#[derive(Copy, Eq, PartialOrd, PartialEq, Hash)]
pub enum TokenType {
    Eof = 0,          // end of file
    LineStart,        // emitted when a line starts
    LineEnd,          // emitted when a line ends
    InvalidStatement, // any invalid statement
    Element,          // any element during element parsing
    Label,            // label is emitted when a label is found
    LabelDef,         // label definition is emitted when a new label is found
    Number,           // number is emitted when a number is found
    StringValue,      // stringValue is emitted when a string has been found
}
lazy_static! (
        static ref TOKENTYPE_MAP: HashMap<TokenType, &'static str> = vec![
        (Eof, "EOF"),
        (LineStart, "new line"),
        (InvalidStatement, "invalid statement"),
        (Element, "element"),
        (Label, "label"),
        (LabelDef, "label definition"),
        (Number, "number"),
        (StringValue, "string"),
    ]
    .into_iter()
    .collect();
);

impl ToString for TokenType {
    fn to_string(&self) -> String {
        String::from(*TOKENTYPE_MAP.get(self).unwrap())
    }
}

const Numbers: &str = "1234567890"; // characters representing any decimal number
const HexadecimalNumbers: &str = "1234567890aAbBcCdDeEfF"; // characters representing any hexadecimal
const Alpha: &str = "abcdefghijklmnopqrstuwvxyzABCDEFGHIJKLMNOPQRSTUWVXYZ"; // characters representing alphanumeric

pub struct Token {
    pub typ: TokenType,
    pub lineno: i32,
    pub text: String,
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    tx: &'a Sender<Token>,
    state: Option<stataFn>,
    lineno: i32,
    start: i32,
    pos: i32,
    debug: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &str, tx: &Sender<Token>, debug: bool) -> Self {
        Self{
            input: String::from(source).chars(),
            tx,
            state: Some(lexLine),
            lineno: 0,
            start: 0,
            pos: 0,
            debug
        }
    }

    pub fn run(mut self) {
        self.emit(LineStart);

        loop {
            if self.state == None {
                break;
            }
            self.state = self.state(&self);
        }
        self.emit(Eof);
    }

    fn next(&mut self) -> char {
        if self.input.next() {
            self.width = 0;
            return '0';
        }

        return char::from_u32(0).unwrap();

    }

    fn blod(self) -> String {
        self.input[self.start..self.pos]
    }

    fn emit(&mut self, t: TokenType) {
        let token = Token{
            typ: t,
            lineno: self.lineno,
            text: self.blod()
        };

        if self.debug {
            println!("{} {} {}", token.lineno, token.typ.to_string(), token.text)
        }

        if let Err(e) = self.tx.send(token) {
            println!("{}", e);
        }
        self.start = self.pos;
    }

}

fn lexLine(l: &lexer) -> stateFn {

}