use crate::tokenize::kind::TokenKind;
use crate::tokenize::position::Position;
use std::ptr::null;

#[derive(Debug, Clone)]
pub(crate) struct Token {
    kind: TokenKind,
    pos: Position,
    imm_s: String,
    imm_f: f64,
    imm_i: i64,
    pub(crate) next: Option<Box<Token>>,
}

impl Token {
    pub fn new(kind: TokenKind, pos: Position, imm_s: String, imm_f: f64, imm_i: i64) -> Token {
        return Token {
            kind,
            pos,
            imm_s,
            imm_f,
            imm_i,
            next: None,
        };
    }
}
