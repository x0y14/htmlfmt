use crate::parse::kind::NodeKind;

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub children: Option<Vec<Option<Box<Node>>>>,

    pub imm_s: String,
    // pub imm_f: f64,
    // pub imm_i: i64,
}

impl Node {
    pub fn new(
        kind: NodeKind,
        lhs: Option<Box<Node>>,
        rhs: Option<Box<Node>>,
        children: Option<Vec<Option<Box<Node>>>>,
        s: String,
    ) -> Node {
        return Node {
            kind,
            lhs,
            rhs,
            children,
            imm_s: s,
        };
    }
}
