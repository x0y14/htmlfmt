#[derive(Debug, Clone)]
pub enum NodeKind {
    OpenTag,  // <x>
    CloseTag, // </x>
    // DeclTag,  // <!...>
    SoloTag,    // <x />
    CommentTag, //
    DoctypeTag, //

    Parameter, // key = value
    Key,
    String,
    Text,
}
