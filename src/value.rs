#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value {
    pub value: String,
    pub color: String,
    pub connectors: [Vec<String>; 4]
}