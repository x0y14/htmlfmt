use crate::tokenize::kind::TokenKind;
use crate::tokenize::token::Token;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unexpected token (expect: {expected:?}, found {found:?})")]
    UnexpectedToken { expected: TokenKind, found: Token },
    #[error("unexpected text (expected: {expected:?}, found: {found:?})")]
    UnexpectedText { expected: String, found: String },
    #[error("unknown parse error")]
    Unknown,
}
