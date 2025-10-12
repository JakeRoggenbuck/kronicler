use super::constants::{DATA_DIRECTORY, PAGE_SIZE};
use super::row::FieldType;
use log::info;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub type PageID = usize;

#[derive(Debug)]
pub struct Page {
    pid: PageID,
    data: Option<[u8; PAGE_SIZE]>,
    index: usize,
    // Either 16 or 64
    field_type_size: usize,
    column_index: usize,
}

impl Page {
    pub fn open(&mut self) {
        let path = self.get_page_path();

        if !path.exists() {
            self.write_page();
        }

        let data = self.read_page();

        self.data = Some(data);

        info!("Wrote page data.");
    }

    pub fn new(pid: PageID, column_index: usize, field_type_size: usize) -> Self {
        Page {
            pid,
            data: None,
            index: 0,
            field_type_size,
            column_index,
        }
    }

    pub fn get_page_path(&self) -> PathBuf {
        Path::new("./")
            .join(DATA_DIRECTORY)
            .join(format!("page_{}_{}.data", self.column_index, self.pid))
    }

    /// Write a whole page to disk
    ///
    /// ```rust
    /// use kronicler::bufferpool::Bufferpool;
    /// use kronicler::row::FieldType;
    ///
    /// let mut bpool = Bufferpool::new(0);
    ///
    /// // TODO: Fix tests
    /// // let page_arc = bpool.create_page(0, FieldType::Epoch(0));
    /// // let mut page = page_arc.lock().unwrap();
    ///
    /// // Set the 0th value to 100
    /// // page.set_value(0, kronicler::row::FieldType::Epoch(100));
    /// // page.write_page();
    /// ```
    ///
    /// Write functions are for writing a Page from disk and not changing any state
    pub fn write_page(&self) {
        let filename = self.get_page_path();

        // TODO: It should not create a new file each time right?
        let file = File::create(&filename);

        match file {
            Ok(mut fp) => {
                // This is the fastest way to do this
                // I do not know all of the conditions that are needed to make this not break
                // TODO: Prove that this works always
                if let Some(d) = self.data {
                    let bytes: [u8; PAGE_SIZE] = unsafe { std::mem::transmute(d) };

                    info!("Writing data to {:?}.", filename);
                    // TODO: Use this result
                    fp.write_all(&bytes).expect("Should be able to write.");
                }
            }
            Err(..) => {
                println!("Error: Cannot open database file.");
            }
        }
    }

    /// Read a page from disk
    ///
    /// ```rust
    /// use kronicler::bufferpool::Bufferpool;
    /// use kronicler::row::FieldType;
    ///
    /// let mut bpool = Bufferpool::new(0);
    ///
    /// // TODO: Fix tests
    /// // let page_arc = bpool.create_page(0, FieldType::Epoch(0));
    /// // let mut page = page_arc.lock().unwrap();
    ///
    /// // Set the 0th value to 100
    /// // page.set_value(0, kronicler::row::FieldType::Epoch(100));
    /// // page.write_page();
    ///
    /// // TODO: Add a read page and test it
    /// ```
    ///
    /// Read functions are for pulling a Page from disk and does not mutate the state of the Page
    pub fn read_page(&self) -> [u8; PAGE_SIZE] {
        let filename = self.get_page_path();

        info!("Trying to open {:?}", filename);
        let mut file = File::open(filename).expect("Should open file.");
        let mut buf: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

        let _ = file.read(&mut buf[..]).expect("Should read.");

        // TODO: Make sure this works as expected always
        // TODO: Maybe this is not needed if input and output is the same type
        // This changed later, so maybe I can remove this now
        let values: [u8; PAGE_SIZE] = unsafe { std::mem::transmute(buf) };
        return values;
    }

    /// Set functions are for changing internal state of a Page
    pub fn set_value(&mut self, index: usize, value: FieldType) {
        if let Some(d) = &mut self.data {
            match value {
                FieldType::Name(a) => {
                    let mut i = 0;

                    for v in a {
                        d[index + i] = v;
                        i += 1;
                    }
                }
                FieldType::Epoch(a) => {
                    let mut i = 0;

                    let bytes = a.to_le_bytes();

                    for v in bytes {
                        d[index + i] = v;
                        i += 1;
                    }
                }
            }
        }

        self.index += self.field_type_size;
    }

    /// Set functions are for changing internal state of a Page
    pub fn set_all_values(&mut self, input: [u8; PAGE_SIZE]) {
        self.data = Some(input);
    }

    /// Get functions are for getting internal state of a Page
    pub fn get_value(&self, index: usize) -> Option<FieldType> {
        if let Some(d) = self.data {
            let mut vals = [0u8; 64];

            for i in 0..self.field_type_size {
                vals[i] = d[index + i];
            }

            // TODO: Don't hardcode sizes of data like this
            if self.field_type_size == 16 {
                let mut b: [u8; 16] = [0; 16];

                let mut i = 0;
                for v in &vals[0..16] {
                    b[i] = *v;
                    i += 1;
                }

                let a: u128 = unsafe { std::mem::transmute(b) };
                return Some(FieldType::Epoch(a));
            }

            // TODO: Don't hardcode sizes of data like this
            if self.field_type_size == 64 {
                return Some(FieldType::Name(vals));
            }
        }

        None
    }

    pub fn size(&self) -> usize {
        self.index
    }

    pub fn capacity(&self) -> usize {
        PAGE_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use crate::row::create_function_name;

    fn cleanup_test_page(page: &Page) {
        let path = page.get_page_path();
        if path.exists() {
            let _ = fs::remove_file(path);
        }
    }

    fn ensure_data_directory() {
        let path = Path::new("./").join(DATA_DIRECTORY);
        if !path.exists() {
            let _ = fs::create_dir_all(path);
        }
    }

    #[test]
    fn test_new_page_creation() {
        let page = Page::new(0, 0, 16);

        assert_eq!(page.pid, 0);
        assert_eq!(page.column_index, 0);
        assert_eq!(page.field_type_size, 16);
        assert_eq!(page.index, 0);
        assert!(page.data.is_none());
    }

    #[test]
    fn test_page_path_generation() {
        let page = Page::new(5, 3, 16);
        let path = page.get_page_path();

        assert!(path.to_string_lossy().contains("page_3_5.data"));
    }

    #[test]
    fn test_open_creates_file_if_not_exists() {
        ensure_data_directory();
        let mut page = Page::new(100, 0, 16);

        page.open();

        assert!(page.get_page_path().exists());
        assert!(page.data.is_some());

        cleanup_test_page(&page);
    }

    #[test]
    fn test_set_and_get_epoch_value() {
        let mut page = Page::new(0, 0, 16);
        page.data = Some([0u8; PAGE_SIZE]);

        let epoch_value = FieldType::Epoch(12345);
        page.set_value(0, epoch_value);

        let retrieved = page.get_value(0);
        assert!(retrieved.is_some());

        if let Some(FieldType::Epoch(val)) = retrieved {
            assert_eq!(val, 12345);
        } else {
            panic!("Expected Epoch value");
        }
    }

    #[test]
    fn test_set_and_get_name_value() {
        let mut page = Page::new(0, 0, 64);
        page.data = Some([0u8; PAGE_SIZE]);

        let mut name_bytes = [0u8; 64];
        let name_str = "TestName";
        let bytes = name_str.as_bytes();
        name_bytes[..bytes.len()].copy_from_slice(bytes);

        page.set_value(0, FieldType::Name(name_bytes));

        let retrieved = page.get_value(0);
        assert!(retrieved.is_some());

        if let Some(FieldType::Name(val)) = retrieved {
            assert_eq!(&val[..bytes.len()], bytes);
        } else {
            panic!("Expected Name value");
        }
    }

    #[test]
    fn test_multiple_epoch_values() {
        let mut page = Page::new(0, 0, 16);
        page.data = Some([0u8; PAGE_SIZE]);

        // Set multiple epoch values
        page.set_value(0, FieldType::Epoch(100));
        page.set_value(16, FieldType::Epoch(200));
        page.set_value(32, FieldType::Epoch(300));

        // Retrieve and verify
        assert_eq!(page.get_value(0), Some(FieldType::Epoch(100)));
        assert_eq!(page.get_value(16), Some(FieldType::Epoch(200)));
        assert_eq!(page.get_value(32), Some(FieldType::Epoch(300)));
    }

    #[test]
    fn test_set_all_values() {
        let mut page = Page::new(0, 0, 16);
        let mut test_data = [0u8; PAGE_SIZE];

        // Set some known pattern
        for i in 0..100 {
            test_data[i] = (i % 256) as u8;
        }

        page.set_all_values(test_data);

        assert!(page.data.is_some());
        if let Some(data) = page.data {
            for i in 0..100 {
                assert_eq!(data[i], (i % 256) as u8);
            }
        }
    }

    #[test]
    fn test_page_size_tracking() {
        let mut page = Page::new(0, 0, 16);
        page.data = Some([0u8; PAGE_SIZE]);

        assert_eq!(page.size(), 0);

        page.set_value(0, FieldType::Epoch(100));
        assert_eq!(page.size(), 16);

        page.set_value(16, FieldType::Epoch(200));
        assert_eq!(page.size(), 32);
    }

    #[test]
    fn test_page_capacity() {
        let page = Page::new(0, 0, 16);
        assert_eq!(page.capacity(), PAGE_SIZE);
    }

    #[test]
    fn test_write_and_read_page() {
        ensure_data_directory();
        let mut page = Page::new(200, 1, 16);
        let mut test_data = [0u8; PAGE_SIZE];

        // Create a pattern to verify
        for i in 0..1000 {
            test_data[i] = ((i * 7) % 256) as u8;
        }

        page.set_all_values(test_data);
        page.write_page();

        // Create a new page and read from disk
        let mut page2 = Page::new(200, 1, 16);
        page2.open();

        if let Some(read_data) = page2.data {
            for i in 0..1000 {
                assert_eq!(read_data[i], ((i * 7) % 256) as u8);
            }
        } else {
            panic!("Expected data to be loaded");
        }

        cleanup_test_page(&page);
    }

    #[test]
    fn test_get_value_without_data() {
        let page = Page::new(0, 0, 16);
        let result = page.get_value(0);
        assert!(result.is_none());
    }

    #[test]
    fn test_epoch_value_persistence() {
        ensure_data_directory();
        let mut page = Page::new(300, 2, 16);
        page.data = Some([0u8; PAGE_SIZE]);

        // Set multiple epoch values
        let values = vec![12345u128, 67890, 11111, 22222, 33333];
        for (i, &val) in values.iter().enumerate() {
            page.set_value(i * 16, FieldType::Epoch(val));
        }

        page.write_page();

        // Read back from disk
        let mut page2 = Page::new(300, 2, 16);
        page2.open();

        // Verify all values
        for (i, &expected) in values.iter().enumerate() {
            if let Some(FieldType::Epoch(val)) = page2.get_value(i * 16) {
                assert_eq!(val, expected);
            } else {
                panic!("Expected Epoch value at index {}", i);
            }
        }

        cleanup_test_page(&page);
    }

    #[test]
    fn test_name_value_persistence() {
        ensure_data_directory();
        let mut page = Page::new(400, 3, 64);
        page.data = Some([0u8; PAGE_SIZE]);

        let names = vec!["Alice", "Bob", "Charlie", "Diana"];

        for (i, &name) in names.iter().enumerate() {
            let name_bytes = create_function_name(name);
            page.set_value(i * 64, FieldType::Name(name_bytes));
        }

        page.write_page();

        // Read back from disk
        let mut page2 = Page::new(400, 3, 64);
        page2.open();

        // Verify all names
        for (i, &expected_name) in names.iter().enumerate() {
            if let Some(FieldType::Name(val)) = page2.get_value(i * 64) {
                let expected_bytes = expected_name.as_bytes();
                assert_eq!(&val[..expected_bytes.len()], expected_bytes);
            } else {
                panic!("Expected Name value at index {}", i);
            }
        }

        cleanup_test_page(&page);
    }

    #[test]
    fn test_different_column_indices() {
        ensure_data_directory();
        let mut page1 = Page::new(500, 0, 16);
        let mut page2 = Page::new(500, 1, 16);

        page1.data = Some([1u8; PAGE_SIZE]);
        page2.data = Some([2u8; PAGE_SIZE]);

        page1.write_page();
        page2.write_page();

        // Verify different files were created
        let path1 = page1.get_page_path();
        let path2 = page2.get_page_path();

        assert_ne!(path1, path2);
        assert!(path1.exists());
        assert!(path2.exists());

        cleanup_test_page(&page1);
        cleanup_test_page(&page2);
    }

    #[test]
    fn test_large_epoch_values() {
        let mut page = Page::new(0, 0, 16);
        page.data = Some([0u8; PAGE_SIZE]);

        let max_value = u128::MAX;
        page.set_value(0, FieldType::Epoch(max_value));

        if let Some(FieldType::Epoch(val)) = page.get_value(0) {
            assert_eq!(val, max_value);
        } else {
            panic!("Expected Epoch value");
        }
    }

    #[test]
    fn test_empty_name_value() {
        let mut page = Page::new(0, 0, 64);
        page.data = Some([0u8; PAGE_SIZE]);

        let name_bytes = [0u8; 64];
        page.set_value(0, FieldType::Name(name_bytes));

        if let Some(FieldType::Name(val)) = page.get_value(0) {
            assert_eq!(val, name_bytes);
        } else {
            panic!("Expected Name value");
        }
    }

    #[test]
    fn test_sequential_writes() {
        let mut page = Page::new(0, 0, 16);
        page.data = Some([0u8; PAGE_SIZE]);

        let initial_size = page.size();

        // Write several values sequentially
        for i in 0..10 {
            let current_index = page.index;
            page.set_value(current_index, FieldType::Epoch(i as u128));
        }

        assert_eq!(page.size(), initial_size + (10 * 16));
    }

    #[test]
    fn test_page_reopen() {
        ensure_data_directory();
        let mut page = Page::new(600, 4, 16);
        page.data = Some([0u8; PAGE_SIZE]);

        page.set_value(0, FieldType::Epoch(999));
        page.write_page();

        // Open the same page again
        let mut page2 = Page::new(600, 4, 16);
        page2.open();

        if let Some(FieldType::Epoch(val)) = page2.get_value(0) {
            assert_eq!(val, 999);
        } else {
            panic!("Expected Epoch value");
        }

        cleanup_test_page(&page);
    }
}
