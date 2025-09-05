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
