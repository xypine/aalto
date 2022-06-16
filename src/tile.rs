use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile {
    pub possible: Vec<Value>
}