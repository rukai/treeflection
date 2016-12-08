#[derive(Debug, PartialEq, Clone)]
pub enum NodeToken {
    ChainProperty (String),
    ChainIndex (usize),
    ChainKey (String),
    ChainContext,
    Help,
    Edit,
    Get,
    Set (String),
    SetDefault,
    SetVariant (String),
    CopyFrom,
    PasteTo,
    Insert (usize),
    Remove (usize),
    Custom (String),
}
