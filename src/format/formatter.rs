use crate::format::config::Config;
use crate::parse::kind::NodeKind;
use crate::parse::node::Node;
use std::fmt::format;

pub struct Formatter {
    config: Config,
}

impl Formatter {
    pub fn new(config: Config) -> Formatter {
        Formatter { config }
    }

    fn str_parameter(&self, node: Box<Node>) -> String {
        return format!(
            "{}=\"{}\"",
            node.lhs.unwrap().imm_s,
            node.rhs.unwrap().imm_s
        );
    }

    fn str_parameters(&self, node: Box<Node>) -> String {
        if node.params.is_none() {
            return "".to_string();
        }

        let mut s: String = "".to_string();
        for node in node.params {
            s += &*self.str_parameter(node);
        }

        return s;
    }

    fn str_solo_tag(&self, node: Box<Node>) -> String {
        let mut params: String = "".to_string();

        if node.params.is_some() {
            params = self.str_parameters(node.params.unwrap())
        }
        if params != "" {
            params = " ".to_string() + &*params;
        }
        return format!("<{}{} />", node.imm_s, params);
    }

    fn str_tag(&self, node: Box<Node>) -> String {
        let mut params: String = "".to_string();
        if node.params.is_some() {
            for param in node.params {
                params += &*self.str_parameter(param);
            }
        }
        if params != "" {
            params = " ".to_string() + &*params;
        }

        return format!("<{}{}>", node.imm_s, params);
    }

    pub fn format(&self, nodes: Vec<Option<Box<Node>>>) -> String {
        // ->->
        let mut left_side: Vec<(String, bool)> = vec![];
        // <-<-
        let mut right_side: Vec<String> = vec![];

        for node in nodes {
            if node.as_ref().is_none() {
                continue;
            }

            match node.as_ref().unwrap().kind {
                NodeKind::Text => {
                    left_side.push((node.unwrap().imm_s, false));
                }
                NodeKind::CommentTag => {
                    left_side.push((format!("<!--{}-->", node.unwrap().imm_s), false));
                }
                NodeKind::DoctypeTag => {
                    left_side.push((format!("<!doctype {}>", node.unwrap().imm_s), false));
                }
                NodeKind::SoloTag => left_side.push((self.str_solo_tag(node.unwrap()), false)),
                NodeKind::Tag => {
                    left_side.push((self.str_tag(node.clone().unwrap()), true));
                    let mut children: String = "".to_string();
                    if node.clone().unwrap().children.is_some() {
                        children = self.format(node.clone().unwrap().children.unwrap());
                    }
                    if children != "" {
                        left_side.push((children, false));
                    }
                    right_side.push(format!("</{}>", node.unwrap().imm_s));
                }
                _ => {}
            }
        }

        let mut result: String = "".to_string();
        let ws: String = " ".repeat(self.config.ident);

        let mut deep: usize = 0;
        for (left, need_dive) in left_side {
            result += &*(ws.clone().to_string().repeat(deep) + left.as_str() + "\n");
            if need_dive {
                deep += 1;
            }
        }
        for right in right_side {
            result += &*(ws.clone().to_string().repeat(deep) + right.as_str() + "\n");
            deep -= 1;
        }

        return result;
    }
}

#[cfg(test)]
mod test {
    use crate::format::config::Config;
    use crate::format::formatter::Formatter;
    use crate::parse::node::Node;
    use crate::parse::parser::Parser;
    use crate::tokenize::tokenizer;

    #[test]
    fn format_1() {
        let html = "<!DOCTYPE html> \
            <html>\
            <body>\
            <h1>hello</h1>\
            <img src=\"https://google.com\"/>\
            </body>\
            </html>";

        let mut tokenizer_ = tokenizer::Tokenizer::new(html);
        let tok = tokenizer_.tokenize();

        let mut parser_ = Parser::new();
        let nodes_res = parser_.parse(tok);

        let mut nodes: Vec<Option<Box<Node>>> = vec![];
        match nodes_res {
            Ok(n) => nodes = n.unwrap(),
            Err(err) => println!("{:#?}", err),
        }

        let formatter_ = Formatter::new(Config::default());
        let s = formatter_.format(nodes.clone());
        println!("{}", s);
    }
}
