use std::path::{Path, PathBuf};
use super::constants::{DATA_DIRECTORY, PAGE_SIZE};
use std::fs::File;
use std::io::prelude::*;

pub type PageID = usize;

#[derive(Debug)]
pub struct Page {
    pid: PageID,
    data: Option<[i64; 512]>,
    index: usize,
}

impl Page {
    pub fn open(&mut self) {
        let data = self.read_page();

        self.data = Some(data);
    }

    pub fn new(pid: PageID) -> Self {
        Page {
            pid,
            data: None,
            index: 0,
        }
    }

    pub fn get_page_path(&self) -> PathBuf {
        Path::new("./")
            .join(DATA_DIRECTORY)
            .join(format!("page_{}.data", self.pid))
    }

    /// Write a whole page to disk
    ///
    /// ```rust
    /// use bufferpool::Bufferpool;
    ///
    /// let mut bpool = Bufferpool::new();
    ///
    /// let page_arc = bpool.create_page();
    /// let mut page = page_arc.lock().unwrap();
    ///
    /// // Set the 0th value to 100
    /// page.set_value(0, 100);
    /// page.write_page();
    /// ```
    ///
    /// Write functions are for writing a Page from disk and not changing any state
    pub fn write_page(&self) {
        let filename = self.get_page_path();
        let file = File::create(filename);

        match file {
            Ok(mut fp) => {
                // This is the fastest way to do this
                // I do not know all of the conditions that are needed to make this not break
                // TODO: Prove that this works always
                if let Some(d) = self.data {
                    let bytes: [u8; PAGE_SIZE] = unsafe { std::mem::transmute(d) };
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
    /// use bufferpool::Bufferpool;
    ///
    /// let mut bpool = Bufferpool::new();
    ///
    /// let page_arc = bpool.create_page();
    /// let mut page = page_arc.lock().unwrap();
    ///
    /// // Set the 0th value to 100
    /// page.set_value(0, 100);
    /// page.write_page();
    ///
    /// // TODO: Add a read page and test it
    /// ```
    ///
    /// Read functions are for pulling a Page from disk and does not mutate the state of the Page
    pub fn read_page(&self) -> [i64; 512] {
        let filename = self.get_page_path();
        let mut file = File::open(filename).expect("Should open file.");
        let mut buf: [u8; 4096] = [0; PAGE_SIZE];

        let _ = file.read(&mut buf[..]).expect("Should read.");

        // TODO: Make sure this works as expected always
        let values: [i64; 512] = unsafe { std::mem::transmute(buf) };
        return values;
    }

    /// Set functions are for changing internal state of a Page
    pub fn set_value(&mut self, index: usize, value: i64) {
        if let Some(d) = &mut self.data {
            d[index] = value;
        }

        self.index += 1;
    }

    /// Set functions are for changing internal state of a Page
    pub fn set_all_values(&mut self, input: [i64; 512]) {
        self.data = Some(input);
    }

    /// Get functions are for getting internal state of a Page
    pub fn get_value(&self, index: usize) -> Option<i64> {
        if let Some(d) = self.data {
            return Some(d[index]);
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

