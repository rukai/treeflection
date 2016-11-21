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
    CopyFrom,
    PasteTo,
    Custom (String),
}
