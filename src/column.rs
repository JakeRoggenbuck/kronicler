use super::capture::Epoch;
use super::metadata::Metadata;

enum Value {
    Epoch(Epoch),
    Name([u8; 64]),
}

/// Used to safe the state of the Column struct
pub struct ColumnMetadata {
    pub current_index: usize,
    pub name: String,
}

/// Implement column specific traits
impl ColumnMetadata {
    fn new(name: String) -> Self {
        ColumnMetadata {
            current_index: 0,
            name,
        }
    }
}

pub struct Column {
    metadata: ColumnMetadata,
}

/// Implement common traits from Metadata
impl Metadata for Column {
    fn save() {
        // Write self.metadata to a file
        todo!();
    }

    fn load() {
        // Load self.metadata from a file if it exists
        todo!();
    }
}

impl Column {
    pub fn insert(value: Value) {}

    pub fn fetch() {}

    pub fn new(name: String) -> Self {
        Column {
            metadata: ColumnMetadata::new(name),
        }
    }
}
