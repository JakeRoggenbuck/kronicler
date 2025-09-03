use super::bufferpool::Bufferpool;
use super::metadata::Metadata;
use super::value::Value;
use std::sync::{Arc, RwLock};

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
    bufferpool: Arc<RwLock<Bufferpool>>,
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
    pub fn insert(&mut self, value: Value) {
        let i = self.metadata.current_index;

        let mut bp = self.bufferpool.write().expect("Could write");
        bp.insert(i, value);

        self.metadata.current_index += 1;
    }

    pub fn fetch() {}

    pub fn new(name: String, bufferpool: Arc<RwLock<Bufferpool>>) -> Self {
        Column {
            metadata: ColumnMetadata::new(name),
            bufferpool,
        }
    }
}
