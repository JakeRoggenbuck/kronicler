use super::row::{create_function_name, Epoch, FieldType, Row, RID};
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
        let name_bytes = create_function_name(&self.name);

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
