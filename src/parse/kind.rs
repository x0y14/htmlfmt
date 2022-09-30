#[derive(Debug, Clone)]
pub enum NodeKind {
    Tag,
    // DeclTag,  // <!...>
    SoloTag,    // <x />
    CommentTag, //
    DoctypeTag, //

    Parameter, // key = value
    Parameters,
    Identifier,
    VString,
    Text,
}
