pub type RID = usize;
pub type Epoch = u128;

#[derive(Debug, Eq, Clone, PartialEq, Ord, PartialOrd)]
pub enum FieldType {
    Name([u8; 64]),
    Epoch(Epoch),
}

impl FieldType {
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
