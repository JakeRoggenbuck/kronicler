use super::capture::Epoch;

/// Used to safe the state of the Column struct
pub struct ColumnMetadata {
    pub current_index: usize,
}

trait Metadata {
    fn save();
    fn load();
    fn new() -> Self;
}

impl Metadata for ColumnMetadata {
    fn new() -> Self {
        ColumnMetadata { current_index: 0 }
    }

    fn save() {}
    fn load() {}
}

pub struct Column {
    metadata: ColumnMetadata,
}

enum Value {
    Epoch(Epoch),
    Name([u8; 64]),
}

impl Column {
    pub fn insert(value: Value) {}

    pub fn fetch() {}

    pub fn new() -> Self {
        Column {
            metadata: ColumnMetadata::new(),
        }
    }
}
