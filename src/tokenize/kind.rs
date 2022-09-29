use crate::tokenize::kind::TokenKind::{
    Amp, Assign, Exclamation, Hyphen, Illegal, Slash, TagBegin, TagEnd,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Illegal,
    Eof,
    Whitespace,

    TagBegin,    // <
    TagEnd,      // >
    Exclamation, // !
    Assign,      // =
    Hyphen,      // -
    Slash,       // /
    Amp,         // &

    String,
    Integer,
    Decimal,

    Text,
}

pub fn symbol_kind(symbol: &str) -> TokenKind {
    match symbol {
        "<" => TagBegin,
        ">" => TagEnd,
        "!" => Exclamation,
        "=" => Assign,
        "-" => Hyphen,
        "/" => Slash,
        "&" => Amp,
        _ => Illegal,
    }
}
