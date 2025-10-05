use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

pub type RID = usize;
pub type Epoch = u128;

#[pyclass]
#[derive(Debug, Eq, Clone, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum FieldType {
    #[serde(with = "BigArray")]
    Name([u8; 64]),
    Epoch(Epoch),
}

#[pymethods]
impl FieldType {
    fn __repr__(&self) -> String {
        match self {
            FieldType::Name(arr) => {
                let name = arr
                    .iter()
                    .take_while(|&&c| c != 0)
                    .map(|&c| c as char)
                    .collect::<String>();

                format!("FieldType::Name(\"{}\")", name)
            }
            FieldType::Epoch(e) => format!("FieldType::Epoch({})", e),
        }
    }

    fn __str__(&self) -> String {
        match self {
            FieldType::Name(arr) => arr
                .iter()
                .take_while(|&&c| c != 0)
                .map(|&c| c as char)
                .collect::<String>(),

            FieldType::Epoch(e) => e.to_string(),
        }
    }

    fn __dict__<'py>(&self, py: Python<'py>) -> Bound<'py, PyDict> {
        let dict = PyDict::new(py);

        match self {
            FieldType::Name(arr) => {
                let name: String = arr
                    .iter()
                    .take_while(|&&c| c != 0)
                    .map(|&c| c as char)
                    .collect();

                dict.set_item("type", "Name").unwrap();
                dict.set_item("value", name).unwrap();
            }
            FieldType::Epoch(e) => {
                dict.set_item("type", "Epoch").unwrap();
                dict.set_item("value", *e).unwrap();
            }
        }

        dict
    }
}

impl FieldType {
    // TODO: Use to_string trait
    pub fn to_string(&self) -> String {
        match self {
            FieldType::Name(a) => {
                let mut name_vec = vec![];

                for i in 0..64 {
                    let c = a[i];

                    if c == 0 {
                        break;
                    }

                    name_vec.push(c);
                }

                return std::str::from_utf8(&name_vec)
                    .expect("Find string.")
                    .to_string();
            }
            FieldType::Epoch(a) => {
                return a.to_string();
            }
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            FieldType::Name(_) => 64,
            FieldType::Epoch(_) => 16,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub struct Row {
    #[pyo3(get)]
    pub id: RID,
    #[pyo3(get)]
    pub fields: Vec<FieldType>,
}

impl Row {
    pub fn new(id: RID, fields: Vec<FieldType>) -> Self {
        Row { id, fields }
    }

    pub fn get_delta(&self) -> u128 {
        let delta = self.fields[3].clone();

        match delta {
            FieldType::Epoch(a) => return a,
            _ => unreachable!(),
        }
    }

    // TODO: Use to_string trait
    pub fn to_string(&self) -> String {
        let name = self.fields[0].to_string();
        let start = self.fields[1].clone();
        let end = self.fields[2].clone();
        let delta = self.fields[3].clone();

        format!(
            "Row {{ id: {}, fields: [\"{}\", {:?}, {:?}, {:?}]}}",
            self.id, name, start, end, delta
        )
    }
}

#[pymethods]
impl Row {
    pub fn __dict__<'py>(&self, py: Python<'py>) -> Bound<'py, PyDict> {
        let dict = PyDict::new(py);

        let field_dicts: Vec<Bound<'py, PyDict>> =
            self.fields.iter().map(|f| f.__dict__(py)).collect();

        dict.set_item("id", self.id).unwrap();
        dict.set_item("fields", field_dicts).unwrap();

        dict
    }

    fn __str__(&self) -> String {
        let name = self.fields[0].to_string();
        let start = self.fields[1].clone();
        let end = self.fields[2].clone();
        let delta = self.fields[3].clone();

        format!(
            "Row(id={}, fields=[\"{}\", {:?}, {:?}, {:?}])",
            self.id, name, start, end, delta
        )
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}
