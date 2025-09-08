use super::row::{Epoch, FieldType, Row, RID};
use pyo3::prelude::*;

#[derive(Debug)]
pub struct Capture {
    pub name: String,
    pub args: Vec<PyObject>,
    pub start: Epoch,
    pub end: Epoch,
    pub delta: Epoch,
}

impl Capture {
    pub fn to_row(&self, id: RID) -> Row {
        let mut name_bytes = [0u8; 64];
        let name_str = self.name.as_bytes();
        name_bytes[..name_str.len()].copy_from_slice(name_str);

        Row {
            id,
            fields: vec![
                FieldType::Name(name_bytes),
                FieldType::Epoch(self.start),
                FieldType::Epoch(self.end),
                FieldType::Epoch(self.delta),
            ],
        }
    }
}
