use crate::tokenize::kind::TokenKind;
use crate::tokenize::token::Token;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unexpected token (expect: {expected:?}, found {found:?})")]
    UnexpectedToken { expected: TokenKind, found: Token },
    #[error("unexpected text (expected: {expected:?}, found: {found:?})")]
    UnexpectedText { expected: String, found: String },
    #[error("opening tag & closing tag name was miss matched (open: {open:?}, close: {close:?})")]
    TagMissMatch { open: String, close: String },
    #[error("unknown parse error")]
    Unknown,
}
