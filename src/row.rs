pub type RID = usize;
pub type Epoch = u128;

#[derive(Debug, Eq, Clone, PartialEq, Ord, PartialOrd)]
pub enum FieldType {
    Name([u8; 64]),
    Epoch(Epoch),
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
