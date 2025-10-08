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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::row::create_function_name;
    use std::fs;

    fn cleanup_test_file(column_index: usize) {
        let filepath = format!("{}/column-{}.data", DATA_DIRECTORY, column_index);
        let _ = fs::remove_file(filepath);
    }

    #[test]
    fn column_metadata_new() {
        let field_type = FieldType::Epoch(0);
        let metadata = ColumnMetadata::new("test_column".to_string(), 5, field_type.clone());

        assert_eq!(metadata.column_index, 5);
        assert_eq!(metadata.current_index, 0);
        assert_eq!(metadata.name, "test_column");
        assert_eq!(metadata.field_type, field_type);
    }

    #[test]
    fn column_metadata_with_name_field() {
        let field_type = FieldType::Name(create_function_name("function"));
        let metadata = ColumnMetadata::new("name_column".to_string(), 1, field_type.clone());

        assert_eq!(metadata.column_index, 1);
        assert_eq!(metadata.name, "name_column");
        assert_eq!(metadata.field_type, field_type);
    }

    #[test]
    fn column_new_creates_fresh_column() {
        let column_index = 999; // Use high number to avoid conflicts
        cleanup_test_file(column_index);

        let bufferpool = Arc::new(RwLock::new(Bufferpool::new(column_index + 1)));
        let field_type = FieldType::Epoch(0);

        let column = Column::new(
            "test".to_string(),
            column_index,
            bufferpool,
            field_type.clone(),
        );

        assert_eq!(column.metadata.column_index, column_index);
        assert_eq!(column.metadata.current_index, 0);
        assert_eq!(column.metadata.name, "test");
        assert_eq!(column.metadata.field_type, field_type);

        cleanup_test_file(column_index);
    }

    #[test]
    fn column_insert_increments_index() {
        let column_index = 1000;
        cleanup_test_file(column_index);

        let bufferpool = Arc::new(RwLock::new(Bufferpool::new(column_index + 1)));
        let mut column = Column::new(
            "counter".to_string(),
            column_index,
            bufferpool,
            FieldType::Epoch(0),
        );

        assert_eq!(column.metadata.current_index, 0);

        column.insert(&FieldType::Epoch(100));
        assert_eq!(column.metadata.current_index, 1);

        column.insert(&FieldType::Epoch(200));
        assert_eq!(column.metadata.current_index, 2);

        column.insert(&FieldType::Epoch(300));
        assert_eq!(column.metadata.current_index, 3);

        cleanup_test_file(column_index);
    }

    #[test]
    fn column_insert_and_fetch() {
        let column_index = 1001;
        cleanup_test_file(column_index);

        let bufferpool = Arc::new(RwLock::new(Bufferpool::new(column_index + 1)));
        let mut column = Column::new(
            "data".to_string(),
            column_index,
            bufferpool,
            FieldType::Epoch(0),
        );

        let value1 = FieldType::Epoch(42);
        let value2 = FieldType::Epoch(1337);

        column.insert(&value1);
        column.insert(&value2);

        let fetched1 = column.fetch(0);
        let fetched2 = column.fetch(1);

        assert_eq!(fetched1, Some(value1));
        assert_eq!(fetched2, Some(value2));

        cleanup_test_file(column_index);
    }

    #[test]
    fn column_insert_name_field() {
        let column_index = 1002;
        cleanup_test_file(column_index);

        let bufferpool = Arc::new(RwLock::new(Bufferpool::new(column_index + 1)));
        let field_type = FieldType::Name(create_function_name("default"));
        let mut column = Column::new(
            "names".to_string(),
            column_index,
            bufferpool,
            field_type.clone(),
        );

        let name1 = FieldType::Name(create_function_name("function_a"));
        let name2 = FieldType::Name(create_function_name("function_b"));

        column.insert(&name1);
        column.insert(&name2);

        assert_eq!(column.metadata.current_index, 2);

        let fetched1 = column.fetch(0);
        let fetched2 = column.fetch(1);

        assert_eq!(fetched1, Some(name1));
        assert_eq!(fetched2, Some(name2));

        cleanup_test_file(column_index);
    }

    #[test]
    #[should_panic]
    fn column_fetch_nonexistent_index() {
        let column_index = 1003;
        cleanup_test_file(column_index);

        let bufferpool = Arc::new(RwLock::new(Bufferpool::new(column_index + 1)));
        let mut column = Column::new(
            "sparse".to_string(),
            column_index,
            bufferpool,
            FieldType::Epoch(0),
        );

        column.insert(&FieldType::Epoch(100));

        // TODO: This would actually return non, instead of returning 0
        // Try to fetch an index that doesn't exist
        let result = column.fetch(0);
        assert_eq!(result, None);

        cleanup_test_file(column_index);
    }

    #[test]
    fn column_save_and_load() {
        let column_index = 1004;
        cleanup_test_file(column_index);

        let bufferpool = Arc::new(RwLock::new(Bufferpool::new(column_index + 1)));
        let field_type = FieldType::Epoch(0);
        let mut column = Column::new(
            "persistent".to_string(),
            column_index,
            Arc::clone(&bufferpool),
            field_type.clone(),
        );

        // Insert some data
        column.insert(&FieldType::Epoch(111));
        column.insert(&FieldType::Epoch(222));
        column.insert(&FieldType::Epoch(333));

        // Save the column
        column.save();

        // Verify metadata file exists
        assert!(Column::metadata_exists(column_index));

        // Load the metadata
        let loaded_metadata = Column::load(column_index);

        assert_eq!(loaded_metadata.column_index, column_index);
        assert_eq!(loaded_metadata.current_index, 3);
        assert_eq!(loaded_metadata.name, "persistent");
        assert_eq!(loaded_metadata.field_type, field_type);

        cleanup_test_file(column_index);
    }

    #[test]
    fn column_new_loads_existing_metadata() {
        let column_index = 1005;
        cleanup_test_file(column_index);

        let bufferpool = Arc::new(RwLock::new(Bufferpool::new(column_index + 1)));
        let field_type = FieldType::Epoch(0);

        // Create and save a column
        let mut column1 = Column::new(
            "original".to_string(),
            column_index,
            Arc::clone(&bufferpool),
            field_type.clone(),
        );
        column1.insert(&FieldType::Epoch(999));
        column1.save();

        // Create a new column with same index - should load existing metadata
        let column2 = Column::new(
            "should_be_ignored".to_string(),
            column_index,
            Arc::clone(&bufferpool),
            field_type.clone(),
        );

        assert_eq!(column2.metadata.name, "original");
        assert_eq!(column2.metadata.current_index, 1);

        cleanup_test_file(column_index);
    }

    #[test]
    fn column_metadata_exists_false_for_new_column() {
        let column_index = 9999;
        cleanup_test_file(column_index);

        assert!(!Column::metadata_exists(column_index));
    }

    #[test]
    fn column_multiple_inserts_sequential() {
        let column_index = 1006;
        cleanup_test_file(column_index);

        let bufferpool = Arc::new(RwLock::new(Bufferpool::new(column_index + 1)));
        let mut column = Column::new(
            "sequence".to_string(),
            column_index,
            bufferpool,
            FieldType::Epoch(0),
        );

        // Insert 10 sequential values
        for i in 0..10 {
            column.insert(&FieldType::Epoch(i * 10));
        }

        assert_eq!(column.metadata.current_index, 10);

        // Verify all values
        for i in 0..10 {
            let fetched = column.fetch(i as usize);
            assert_eq!(fetched, Some(FieldType::Epoch(i * 10)));
        }

        cleanup_test_file(column_index);
    }

    #[test]
    fn column_field_type_size_epoch() {
        let field_type = FieldType::Epoch(100);
        let metadata = ColumnMetadata::new("test".to_string(), 0, field_type);

        assert_eq!(metadata.field_type.get_size(), 16);
    }

    #[test]
    fn column_field_type_size_name() {
        let field_type = FieldType::Name(create_function_name("test"));
        let metadata = ColumnMetadata::new("test".to_string(), 0, field_type);

        assert_eq!(metadata.field_type.get_size(), 64);
    }
}
