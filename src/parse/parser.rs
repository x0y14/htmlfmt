use crate::parse::err::ParseError;
use crate::parse::kind::NodeKind;
use crate::parse::node::Node;
use crate::tokenize::kind::TokenKind;
use crate::tokenize::token::Token;

pub struct Parser {
    token: Option<Box<Token>>,
}

impl Parser {
    pub fn new() -> Parser {
        return Parser { token: None };
    }

    fn current(&self) -> Box<Token> {
        return self.token.clone().unwrap();
    }

    fn is_eof(&self) -> bool {
        return self.current().kind == TokenKind::Eof;
    }

    fn consume(&mut self) -> Option<Box<Token>> {
        let tok = self.current();
        self.token = self.current().next;
        return Some(tok);
    }

    fn consume_kind(&mut self, kind: TokenKind) -> Option<Box<Token>> {
        if self.current().kind == kind {
            let tok = self.current();
            self.token = self.current().next;
            return Some(tok);
        }
        return None;
    }

    fn expect_kind(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        if self.current().kind == kind {
            let cur = self.current();
            self.token = cur.next.clone();
            return Ok(*cur);
        }
        return Err(ParseError::UnexpectedToken {
            expected: TokenKind::Assign,
            found: *self.current(),
        });
    }

    fn expect_text(&mut self, text: String, case_sensitive: bool) -> Result<(), ParseError> {
        return match self.expect_kind(TokenKind::Text) {
            Err(error) => Err(error),
            Ok(tok) => {
                if *&case_sensitive && (tok.imm_s == text) {
                    return Ok(());
                }
                if *&!case_sensitive && (tok.imm_s.to_lowercase() == text) {
                    return Ok(());
                }
                return Err(ParseError::UnexpectedText {
                    expected: text,
                    found: tok.imm_s,
                });
            }
        };
    }

    fn parse_text(&mut self) -> Result<Option<Box<Node>>, ParseError> {
        let mut text: String = "".to_string();

        while !self.is_eof() {
            match self.consume_kind(TokenKind::Text) {
                None => break,
                Some(tok) => text += &*tok.imm_s,
            }
        }

        let nd = Node::new(NodeKind::Text, None, None, None, text);
        return Ok(Some(Box::from(nd)));
    }

    fn parse_decl_tag(&mut self) -> Result<Option<Box<Node>>, ParseError> {
        // doctype or comment

        // comment
        if self.consume_kind(TokenKind::Hyphen) != None {
            match self.expect_kind(TokenKind::Hyphen) {
                Err(error) => return Err(error),
                Ok(_) => {
                    let mut comment: String = "".to_string();
                    while !self.is_eof() {
                        if self.consume_kind(TokenKind::Hyphen) != None {
                            if self.consume_kind(TokenKind::Hyphen) != None {
                                if self.consume_kind(TokenKind::TagEnd) != None {
                                    // 終わり
                                    return Ok(Some(Box::from(Node::new(
                                        NodeKind::CommentTag,
                                        None,
                                        None,
                                        None,
                                        comment,
                                    ))));
                                } else {
                                    comment += "--";
                                    continue;
                                }
                            } else {
                                comment += "-";
                                continue;
                            }
                        }

                        if self.consume_kind(TokenKind::Whitespace) != None {
                            comment += " ";
                            continue;
                        }

                        comment += &*self.consume().unwrap().imm_s
                    }
                }
            }
        }

        // consume doctype
        match self.expect_text("doctype".to_string(), false) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
        // consume ws
        match self.expect_kind(TokenKind::Whitespace) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        // type: eg. html
        let doctype_node = match self.expect_kind(TokenKind::Text) {
            Ok(tok) => Node::new(
                NodeKind::DoctypeTag,
                None,
                None,
                None,
                tok.imm_s.to_string(),
            ),
            Err(err) => return Err(err),
        };

        // consume ">"
        match self.expect_kind(TokenKind::TagEnd) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        return Ok(Some(Box::from(doctype_node)));
    }

    fn parse_tag(&mut self) -> Result<Option<Box<Node>>, ParseError> {
        // // expect "<"
        // match self.expect_kind(TokenKind::TagBegin) {
        //     Ok(_) => (),
        //     Err(error) => return Err(error),
        // };

        if self.consume_kind(TokenKind::Exclamation) != None {
            return self.parse_decl_tag();
        }

        return Err(ParseError::Unknown);
    }

    pub fn parse(&mut self, token: Box<Token>) -> Result<Vec<Option<Box<Node>>>, ParseError> {
        self.token = Some(token);
        let mut nodes: Vec<Option<Box<Node>>> = Vec::new();

        while !self.is_eof() {
            let nd_result = match self.consume_kind(TokenKind::TagBegin) {
                Some(_) => self.parse_tag(),
                None => self.parse_text(),
            };
            match nd_result {
                Ok(nd) => nodes.push(nd),
                Err(err) => return Err(err),
            }
        }

        return Ok(nodes);
    }
}

#[cfg(test)]
mod test {
    use crate::parse::parser::Parser;
    use crate::tokenize::tokenizer;
    #[test]
    fn parse_1() {
        let mut tokenizer_ = tokenizer::Tokenizer::new("<!doctype html><!-- hello, w--orld -->");
        let tok = tokenizer_.tokenize();

        let mut parser_ = Parser::new();
        let nodes = parser_.parse(tok);
        println!("{:#?}", nodes)
    }
}
