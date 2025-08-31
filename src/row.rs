pub type RID = usize;

#[derive(Debug, Eq, Clone, PartialEq, Ord, PartialOrd)]
pub enum FieldType {
    Int(i64),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub id: RID,
    pub fields: Vec<FieldType>,
}

impl Row {
    pub fn new(id: RID, fields: Vec<FieldType>) -> Self {
        Row { id, fields }
    }
}
