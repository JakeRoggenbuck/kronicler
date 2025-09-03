use super::bufferpool::Bufferpool;
use super::metadata::Metadata;
use super::row::FieldType;
use std::sync::{Arc, RwLock};

/// Used to safe the state of the Column struct
pub struct ColumnMetadata {
    // Which column it is
    pub column_number: usize,
    pub current_index: usize,
    pub name: String,
    pub field_type: FieldType,
}

/// Implement column specific traits
impl ColumnMetadata {
    fn new(name: String, column_number: usize, field_type: FieldType) -> Self {
        ColumnMetadata {
            column_number,
            current_index: 0,
            name,
            field_type,
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
        todo!()
    }

    fn load() {
        // Load self.metadata from a file if it exists
        todo!()
    }
}

impl Column {
    pub fn insert(&mut self, value: &FieldType) {
        let i = self.metadata.current_index;

        let mut bp = self.bufferpool.write().expect("Could write.");
        // Index is auto-incremented
        bp.insert(i, self.metadata.column_number, value);

        self.metadata.current_index += 1;
    }

    pub fn fetch(&self, index: usize) -> Option<FieldType> {
        let field_type_size = match self.metadata.field_type {
            FieldType::Name(_) => 64,
            FieldType::Epoch(_) => 16,
        };

        let bp = self.bufferpool.read().ok()?;
        bp.fetch(index, self.metadata.column_number, field_type_size)
    }

    pub fn new(
        name: String,
        column_number: usize,
        bufferpool: Arc<RwLock<Bufferpool>>,
        field_type: FieldType,
    ) -> Self {
        {
            // let mut bp = bufferpool.write().expect("Should write.");
            // bp.create_column(column_number);
        }

        Column {
            metadata: ColumnMetadata::new(name, column_number, field_type),
            bufferpool,
        }
    }
}
