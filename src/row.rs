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

pub fn create_function_name(s: &str) -> [u8; 64] {
    let mut arr = [0u8; 64];
    let bytes = s.as_bytes();
    let len = bytes.len().min(64);
    arr[..len].copy_from_slice(&bytes[..len]);
    arr
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fieldtype_name_to_string() {
        let name = FieldType::Name(create_function_name("test_function"));
        assert_eq!(name.to_string(), "test_function");
    }

    #[test]
    fn fieldtype_name_to_string_with_nulls() {
        let mut arr = [0u8; 64];
        arr[0] = b't';
        arr[1] = b'e';
        arr[2] = b's';
        arr[3] = b't';
        // Rest are zeros
        let name = FieldType::Name(arr);
        assert_eq!(name.to_string(), "test");
    }

    #[test]
    fn fieldtype_name_get_size() {
        let name = FieldType::Name(create_function_name("any_name"));
        assert_eq!(name.get_size(), 64);
    }

    #[test]
    fn fieldtype_name_str() {
        let name = FieldType::Name(create_function_name("my_function"));
        assert_eq!(name.__str__(), "my_function");
    }

    #[test]
    fn fieldtype_name_repr() {
        let name = FieldType::Name(create_function_name("my_function"));
        assert_eq!(name.__repr__(), "FieldType::Name(\"my_function\")");
    }

    // FieldType::Epoch tests
    #[test]
    fn fieldtype_epoch_to_string() {
        let epoch = FieldType::Epoch(1234567890);
        assert_eq!(epoch.to_string(), "1234567890");
    }

    #[test]
    fn fieldtype_epoch_get_size() {
        let epoch = FieldType::Epoch(999);
        assert_eq!(epoch.get_size(), 16);
    }

    #[test]
    fn fieldtype_epoch_str() {
        let epoch = FieldType::Epoch(42);
        assert_eq!(epoch.__str__(), "42");
    }

    #[test]
    fn fieldtype_epoch_repr() {
        let epoch = FieldType::Epoch(42);
        assert_eq!(epoch.__repr__(), "FieldType::Epoch(42)");
    }

    #[test]
    fn fieldtype_equality() {
        let epoch1 = FieldType::Epoch(100);
        let epoch2 = FieldType::Epoch(100);
        let epoch3 = FieldType::Epoch(200);

        assert_eq!(epoch1, epoch2);
        assert_ne!(epoch1, epoch3);
    }

    #[test]
    fn fieldtype_ordering() {
        let epoch1 = FieldType::Epoch(100);
        let epoch2 = FieldType::Epoch(200);

        assert!(epoch1 < epoch2);
        assert!(epoch2 > epoch1);
    }

    #[test]
    fn fieldtype_clone() {
        let original = FieldType::Epoch(500);
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn row_new() {
        let fields = vec![
            FieldType::Name(create_function_name("test")),
            FieldType::Epoch(1),
            FieldType::Epoch(2),
            FieldType::Epoch(1),
        ];
        let row = Row::new(100, fields.clone());

        assert_eq!(row.id, 100);
        assert_eq!(row.fields, fields);
    }

    #[test]
    fn row_get_delta() {
        let row = Row {
            id: 1,
            fields: vec![
                FieldType::Name(create_function_name("func")),
                FieldType::Epoch(1000),
                FieldType::Epoch(2000),
                FieldType::Epoch(1000), // delta
            ],
        };

        assert_eq!(row.get_delta(), 1000);
    }

    #[test]
    fn row_to_string_test() {
        let r = Row {
            id: 1000,
            fields: vec![
                FieldType::Epoch(1),
                FieldType::Epoch(1),
                FieldType::Epoch(1),
                FieldType::Epoch(1),
            ],
        };

        assert_eq!(
            r.to_string(),
            "Row { id: 1000, fields: [\"1\", Epoch(1), Epoch(1), Epoch(1)]}"
        );
    }

    #[test]
    fn row_to_string_with_name() {
        let r = Row {
            id: 500,
            fields: vec![
                FieldType::Name(create_function_name("my_function")),
                FieldType::Epoch(1000),
                FieldType::Epoch(2000),
                FieldType::Epoch(1000),
            ],
        };

        assert_eq!(
            r.to_string(),
            "Row { id: 500, fields: [\"my_function\", Epoch(1000), Epoch(2000), Epoch(1000)]}"
        );
    }

    #[test]
    fn row_str() {
        let r = Row {
            id: 42,
            fields: vec![
                FieldType::Name(create_function_name("test")),
                FieldType::Epoch(100),
                FieldType::Epoch(200),
                FieldType::Epoch(100),
            ],
        };

        assert_eq!(
            r.__str__(),
            "Row(id=42, fields=[\"test\", Epoch(100), Epoch(200), Epoch(100)])"
        );
    }

    #[test]
    fn row_repr() {
        let r = Row {
            id: 42,
            fields: vec![
                FieldType::Name(create_function_name("test")),
                FieldType::Epoch(100),
                FieldType::Epoch(200),
                FieldType::Epoch(100),
            ],
        };

        assert_eq!(r.__repr__(), r.__str__());
    }

    #[test]
    fn row_clone() {
        let original = Row {
            id: 10,
            fields: vec![FieldType::Epoch(1), FieldType::Epoch(2)],
        };
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }

    #[test]
    fn row_equality() {
        let row1 = Row {
            id: 1,
            fields: vec![FieldType::Epoch(100)],
        };
        let row2 = Row {
            id: 1,
            fields: vec![FieldType::Epoch(100)],
        };
        let row3 = Row {
            id: 2,
            fields: vec![FieldType::Epoch(100)],
        };

        assert_eq!(row1, row2);
        assert_ne!(row1, row3);
    }

    #[test]
    fn fieldtype_empty_name() {
        let name = FieldType::Name([0u8; 64]);
        assert_eq!(name.to_string(), "");
    }

    #[test]
    fn fieldtype_max_length_name() {
        let arr = [b'a'; 64];
        let name = FieldType::Name(arr);
        let result = name.to_string();
        assert_eq!(result.len(), 64);
        assert_eq!(result, "a".repeat(64));
    }

    #[test]
    fn fieldtype_epoch_zero() {
        let epoch = FieldType::Epoch(0);
        assert_eq!(epoch.to_string(), "0");
    }

    #[test]
    fn fieldtype_epoch_large_value() {
        let epoch = FieldType::Epoch(u128::MAX);
        assert_eq!(epoch.to_string(), u128::MAX.to_string());
    }

    #[test]
    fn row_empty_fields() {
        let row = Row::new(1, vec![]);
        assert_eq!(row.id, 1);
        assert_eq!(row.fields.len(), 0);
    }

    #[test]
    fn row_single_field() {
        let row = Row::new(10, vec![FieldType::Epoch(999)]);
        assert_eq!(row.fields.len(), 1);
    }

    #[test]
    fn row_many_fields() {
        let fields = vec![
            FieldType::Epoch(1),
            FieldType::Epoch(2),
            FieldType::Epoch(3),
            FieldType::Epoch(4),
            FieldType::Epoch(5),
        ];
        let row = Row::new(1, fields.clone());
        assert_eq!(row.fields.len(), 5);
    }
}
