#[derive(Debug, PartialEq, Clone)]
pub enum NodeToken {
    ChainProperty (String),
    ChainIndex (usize),
    ChainKey (String),
    ChainContext,
    Get,
    Set (String),
    CopyFrom,
    PasteTo,
    Custom (String),
}
