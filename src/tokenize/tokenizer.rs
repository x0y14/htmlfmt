use crate::tokenize::kind::{symbol_kind, TokenKind};
use crate::tokenize::position::Position;
use crate::tokenize::token::Token;
use std::str::Chars;

struct Tokenizer {
    target: String,
    pos: Position,
}

impl Tokenizer {
    pub fn new(target: &str) -> Tokenizer {
        return Tokenizer {
            target: target.to_string(),
            pos: Position::new(1, 0, 0),
        };
    }

    fn is_eof(&self) -> bool {
        return self.pos.at_whole >= self.target.len() as u32;
    }

    fn move_horizon(&mut self, n: u32) {
        self.pos.at_line += n;
        self.pos.at_whole += n;
    }

    fn move_line(&mut self, n: u32) {
        self.pos.at_line = 0;
        self.pos.at_whole += n;
        self.pos.line_no += n;
    }

    fn current(&self) -> char {
        return self.target.chars().nth(self.pos.at_whole as usize).unwrap();
    }

    fn peek(&self, n: u32) -> char {
        return self
            .target
            .chars()
            .nth((self.pos.at_whole + n) as usize)
            .unwrap();
    }

    fn start_with(&self, word: String) -> bool {
        let chars: Chars = word.chars();
        for (i, c) in chars.enumerate() {
            if self.peek(i as u32) != c {
                return false;
            }
        }
        return true;
    }

    fn is_white(&self) -> bool {
        return self.current() == '\n' || self.current() == '\t' || self.current() == ' ';
    }

    fn is_number(&self) -> bool {
        return self.current() == '0'
            || self.current() == '1'
            || self.current() == '2'
            || self.current() == '3'
            || self.current() == '4'
            || self.current() == '5'
            || self.current() == '6'
            || self.current() == '7'
            || self.current() == '8'
            || self.current() == '9';
    }

    fn is_symbol(&self) -> bool {
        let symbols: Vec<&str> = vec!["<", ">", "!", "=", "-", "/", "&"];
        for s in symbols {
            if self.start_with(s.to_string()) {
                return true;
            }
        }

        return false;
    }

    fn consume_string(&mut self, is_single: bool) -> String {
        let mut s: String = "".to_string();

        // consume start single/double quotation
        self.move_horizon(1);

        while !self.is_eof() {
            let cur = self.current();
            if cur == '\'' && is_single {
                break;
            }
            if cur == '"' && !is_single {
                break;
            }
            s += &*cur.to_string();
            self.move_horizon(1);
        }

        // consume end single/double quotation
        self.move_horizon(1);

        return s;
    }

    fn consume_numeric(&mut self) -> (f64, bool) {
        let mut s: String = "".to_string();
        let mut include_dot: bool = false;

        while !self.is_eof() {
            if self.is_number() {
                s += &*self.current().to_string()
            } else if self.current() == '.' {
                s += &*self.current().to_string();
                include_dot = true;
            } else {
                break;
            }
            self.move_horizon(1);
        }

        return (s.parse().unwrap(), include_dot);
    }

    fn consume_white(&mut self) -> String {
        let mut s: String = "".to_string();

        while !self.is_eof() {
            if self.is_white() && self.current() != '\n' {
                s += &*self.current().to_string();
                self.move_horizon(1);
            } else if self.current() == '\n' {
                s += &*self.current().to_string();
                self.move_line(1);
            } else {
                break;
            }
        }

        return s;
    }

    fn consume_symbol(&mut self) -> String {
        let s: String = self.current().to_string();
        self.move_horizon(1);
        return s;
    }

    fn link_white_token<'a>(
        &mut self,
        cur: &'a mut Token,
        pos: Position,
        ws: String,
    ) -> &'a mut Box<Token> {
        let tok: Token = Token::new(TokenKind::Whitespace, pos.clone(), ws, 0 as f64, 0);
        cur.next = Some(Box::new(tok));
        return cur.next.as_mut().unwrap();
    }

    fn link_symbol_token<'a>(
        &mut self,
        cur: &'a mut Token,
        pos: Position,
        symbol: String,
    ) -> &'a mut Box<Token> {
        let tok: Token = Token::new(
            symbol_kind(symbol.as_str()),
            pos.clone(),
            "".to_string(),
            0 as f64,
            0,
        );
        cur.next = Some(Box::new(tok));
        return cur.next.as_mut().unwrap();
    }

    fn link_eof_token<'a>(&mut self, cur: &'a mut Token, pos: Position) -> &'a mut Box<Token> {
        let tok: Token = Token::new(TokenKind::Eof, pos.clone(), "".to_string(), 0 as f64, 0);
        cur.next = Some(Box::new(tok));
        return cur.next.as_mut().unwrap();
    }

    pub fn tokenize(&mut self) -> Box<Token> {
        let mut head = Token::new(
            TokenKind::Illegal,
            self.pos.clone(),
            "".to_string(),
            0 as f64,
            0,
        );

        let mut cur: &mut Token = &mut head;
        while !self.is_eof() {
            if self.is_white() {
                let ws: String = self.consume_white();
                cur = self.link_white_token(cur, self.pos.clone(), ws);
                continue;
            }
            if self.is_symbol() {
                let sym: String = self.consume_symbol();
                cur = self.link_symbol_token(cur, self.pos.clone(), sym);
                continue;
            }
        }

        cur = self.link_eof_token(cur, self.pos.clone());

        return head.next.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenize::tokenizer::Tokenizer;

    #[test]
    fn tokenize() {
        let input = " <> ";
        let mut tokenizer = Tokenizer::new(input);
        let token = tokenizer.tokenize();
        println!("{:#?}", token)
    }
}
