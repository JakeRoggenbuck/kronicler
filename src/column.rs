use super::bufferpool::Bufferpool;
use super::constants::DATA_DIRECTORY;
use super::filewriter::{build_binary_writer, Writer};
use super::row::FieldType;
use log::info;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, RwLock};

/// Used to safe the state of the Column struct
#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnMetadata {
    // Which column it is
    pub column_index: usize,
    pub current_index: usize,
    pub name: String,
    pub field_type: FieldType,
}

/// Implement column specific traits
impl ColumnMetadata {
    fn new(name: String, column_index: usize, field_type: FieldType) -> Self {
        ColumnMetadata {
            column_index,
            current_index: 0,
            name,
            field_type,
        }
    }
}

pub struct Column {
    pub metadata: ColumnMetadata,
    bufferpool: Arc<RwLock<Bufferpool>>,
}

/// Implement common traits from Metadata
/// TODO: How do I use ./metadata.rs as a trait and then have the return time for `load` be the
/// correct type? Right now, I will just have load and save be their own functions
impl Column {
    pub fn metadata_exists(column_index: usize) -> bool {
        let filepath = format!("{}/column-{}.data", DATA_DIRECTORY, column_index);

        Path::new(&filepath).exists()
    }

    pub fn save(&self) {
        let writer: Writer<ColumnMetadata> = build_binary_writer();
        let filepath = format!(
            "{}/column-{}.data",
            DATA_DIRECTORY, self.metadata.column_index
        );
        info!(
            "Saving Column {} to {}",
            self.metadata.column_index, filepath
        );
        writer.write_file(filepath.as_str(), &self.metadata);
    }

    pub fn load(column_index: usize) -> ColumnMetadata {
        let writer: Writer<ColumnMetadata> = build_binary_writer();
        let filepath = format!("{}/column-{}.data", DATA_DIRECTORY, column_index);

        info!("Loading Column {} to {}", column_index, filepath);
        writer.read_file(filepath.as_str())
    }
}

impl Column {
    pub fn insert(&mut self, value: &FieldType) {
        let i = self.metadata.current_index;

        let mut bp = self.bufferpool.write().expect("Could write.");
        // Index is auto-incremented
        bp.insert(i, self.metadata.column_index, value);

        self.metadata.current_index += 1;
    }

    pub fn fetch(&mut self, index: usize) -> Option<FieldType> {
        info!("Fetching {}", index);
        let field_type_size = self.metadata.field_type.get_size();

        let bufferpool = self.bufferpool.write();

        match bufferpool {
            Ok(mut bp) => return bp.fetch(index, self.metadata.column_index, field_type_size),
            Err(e) => {
                info!("{}", e);
                return None;
            }
        }
    }

    pub fn new(
        name: String,
        column_index: usize,
        bufferpool: Arc<RwLock<Bufferpool>>,
        field_type: FieldType,
    ) -> Self {
        {
            // let mut bp = bufferpool.write().expect("Should write.");
            // bp.create_column(column_index);
        }

        // Use existing metadata if it's around
        if Column::metadata_exists(column_index) {
            return Column {
                metadata: Column::load(column_index),
                bufferpool,
            };
        }

        Column {
            metadata: ColumnMetadata::new(name, column_index, field_type),
            bufferpool,
        }
    }
}
