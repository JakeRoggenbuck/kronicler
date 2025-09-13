use super::bufferpool::Bufferpool;
use super::capture::Capture;
use super::column::Column;
use super::constants::{CONSUMER_DELAY, DATA_DIRECTORY, DB_WRITE_BUFFER_SIZE};
use super::index::Index;
use super::queue::KQueue;
use super::row::{Epoch, FieldType, Row};
use log::{debug, info};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::JoinHandle;
use std::{thread, time};

#[pyclass]
pub struct Database {
    queue: KQueue,
    columns: Arc<Mutex<Vec<Column>>>,
    /// Index rows by `name` field in capture
    name_index: Arc<Mutex<Index>>,
    /// Keep track of the current row being inserted
    row_id: AtomicUsize,
}

#[pymethods]
impl Database {
    #[new]
    pub fn new() -> Self {
        Database::create_data_dir();

        Database::new_reader()
    }

    // Returns immediately while consumer runs in background - Python compatible
    pub fn init(&self) {
        let queue_clone = Arc::clone(&self.queue.queue);
        let columns_clone = Arc::clone(&self.columns);
        let name_index_clone = Arc::clone(&self.name_index);
        let row_id = Arc::new(AtomicUsize::new(self.row_id.load(Ordering::SeqCst)));

        thread::spawn(move || loop {
            Self::consume_capture_threaded(
                queue_clone.clone(),
                columns_clone.clone(),
                name_index_clone.clone(),
                &row_id,
            );

            let timeout = time::Duration::from_millis(CONSUMER_DELAY);
            thread::sleep(timeout);
        });
    }

    #[staticmethod]
    pub fn exists() -> bool {
        Path::new(&DATA_DIRECTORY).exists()
    }

    #[staticmethod]
    pub fn check_for_data() {
        if !Database::exists() {
            eprintln!("Database does not exist at \"{}\".", &DATA_DIRECTORY);
            std::process::exit(0);
        }
    }

    #[staticmethod]
    pub fn new_reader() -> Self {
        Database::check_for_data();

        let column_count = 4;

        let bp = Bufferpool::new(column_count);
        let bufferpool = Arc::new(RwLock::new(bp));

        let name_col = Column::new(
            "name".to_string(),
            0,
            bufferpool.clone(),
            FieldType::Name([0u8; 64]),
        );

        let start_col = Column::new(
            "start".to_string(),
            1,
            bufferpool.clone(),
            FieldType::Epoch(0),
        );

        let end_col = Column::new(
            "end".to_string(),
            2,
            bufferpool.clone(),
            FieldType::Epoch(0),
        );

        let delta_col = Column::new(
            "delta".to_string(),
            3,
            bufferpool.clone(),
            FieldType::Epoch(0),
        );

        let columns = vec![name_col, start_col, end_col, delta_col];

        assert_eq!(columns.len(), column_count);

        for col in &columns {
            col.save();
        }

        let name_index = Index::new();

        Database {
            queue: KQueue::new(),
            columns: Arc::new(Mutex::new(columns)),
            name_index: Arc::new(Mutex::new(name_index)),
            row_id: AtomicUsize::new(0),
        }
    }

    /// Capture a function and write it to the queue
    pub fn capture(&mut self, name: String, args: Vec<PyObject>, start: Epoch, end: Epoch) {
        self.queue.capture(name, args, start, end);
    }

    pub fn fetch(&self, index: usize) -> Option<Row> {
        info!("Starting fetch on index {}", index);

        let mut data = vec![];

        let mut columns = self.columns.lock().unwrap();
        for col in columns.iter_mut() {
            let field = col.fetch(index);

            if let Some(f) = field {
                data.push(f);
            }
        }

        // TODO: Fix this to make it a better check for unwritten data
        if data.len() > 1 && data[1] == FieldType::Epoch(0) {
            return None;
        }

        Some(Row {
            id: index,
            fields: data,
        })
    }

    pub fn fetch_all(&self) -> Vec<Row> {
        let mut all = vec![];

        let mut index = 0;

        loop {
            let row = self.fetch(index);

            if let Some(r) = row {
                all.push(r);
            } else {
                break;
            }

            index += 1;
        }

        all
    }

    pub fn fetch_all_as_dict<'py>(&self, py: Python<'py>) -> Vec<Bound<'py, PyDict>> {
        let a = self.fetch_all().into_iter().map(|x| x.__dict__(py));
        let rows_dict: Vec<Bound<'py, PyDict>> = a.collect();
        rows_dict
    }

    /// Find the average time a function took to run
    pub fn average(&self, function_name: &str) -> Option<f64> {
        let mut name_bytes = [0u8; 64];
        let bytes = function_name.as_bytes();
        name_bytes[..bytes.len()].copy_from_slice(bytes);

        let name_index = self.name_index.lock().unwrap();
        if let Some(ids) = name_index.get(FieldType::Name(name_bytes)) {
            let ids_clone = ids.clone();
            drop(name_index); // Release the lock early

            let mut values = vec![];

            for id in &ids_clone {
                // TODO: This could be optimized
                if let Some(row) = self.fetch(*id) {
                    let delta_index = 3;

                    let fs = row.fields;
                    if fs.len() > delta_index {
                        let f = fs[delta_index].clone();

                        match f {
                            FieldType::Epoch(e) => values.push(e),
                            _ => {}
                        }
                    }
                }
            }

            if !values.is_empty() {
                let sum: u128 = values.iter().sum();
                let avg = sum as f64 / values.len() as f64;

                return Some(avg);
            }
        }

        None
    }
}

impl Database {
    // If you need the JoinHandle for testing, use this method instead
    pub fn init_with_handle(&self) -> JoinHandle<()> {
        let queue_clone = Arc::clone(&self.queue.queue);
        let columns_clone = Arc::clone(&self.columns);
        let name_index_clone = Arc::clone(&self.name_index);
        let row_id = Arc::new(AtomicUsize::new(self.row_id.load(Ordering::SeqCst)));

        thread::spawn(move || loop {
            Self::consume_capture_threaded(
                queue_clone.clone(),
                columns_clone.clone(),
                name_index_clone.clone(),
                &row_id,
            );

            let timeout = time::Duration::from_millis(CONSUMER_DELAY);
            thread::sleep(timeout);
        })
    }

    // New threaded version of consume_capture
    fn consume_capture_threaded(
        queue: Arc<Mutex<VecDeque<Capture>>>,
        columns: Arc<Mutex<Vec<Column>>>,
        name_index: Arc<Mutex<Index>>,
        row_id: &AtomicUsize,
    ) {
        let mut q = queue.lock().unwrap();

        if q.len() > DB_WRITE_BUFFER_SIZE {
            info!("Starting bulk write!");

            while !q.is_empty() {
                let capture = q.pop_front();

                if let Some(c) = capture {
                    // Get the row_id value (prev) and then add one to row_id
                    let prev = row_id.fetch_add(1, Ordering::SeqCst);
                    let row = c.to_row(prev);

                    info!("Writing {:?}...", &row);

                    // Insert each field into its respective column
                    {
                        let mut cols = columns.lock().unwrap();
                        let mut col_index = 0;
                        for field in &row.fields {
                            if col_index < cols.len() {
                                cols[col_index].insert(field);
                                col_index += 1;
                            }
                        }
                    } // Release columns lock

                    // Insert into name index
                    {
                        let mut idx = name_index.lock().unwrap();
                        debug!("{:?}", *idx);
                        idx.insert(row.clone(), 0);
                    } // Release name_index lock
                }
            }
        }
    }

    // Original consume_capture method (kept for compatibility)
    fn consume_capture(&mut self, queue: Arc<Mutex<VecDeque<Capture>>>) {
        let mut q = queue.lock().unwrap();

        if q.len() > DB_WRITE_BUFFER_SIZE {
            info!("Starting bulk write!");

            while !q.is_empty() {
                let capture = q.pop_front();

                if let Some(c) = capture {
                    // Get the self.row_id value (prev) and then add one to self.row_id
                    let prev = self.row_id.fetch_add(1, Ordering::SeqCst);
                    let row = c.to_row(prev);

                    info!("Writing {:?}...", &row);

                    // Insert each field into its respective column
                    let mut col_index = 0;
                    let mut columns = self.columns.lock().unwrap();
                    for field in &row.fields {
                        if col_index < columns.len() {
                            columns[col_index].insert(field);
                            col_index += 1;
                        }
                    }
                    drop(columns);

                    let mut name_index = self.name_index.lock().unwrap();
                    debug!("{:?}", *name_index);
                    name_index.insert(row.clone(), 0);
                }
            }
        }
    }

    fn create_data_dir() {
        fs::create_dir_all(DATA_DIRECTORY)
            .expect(&format!("Could not create directory '{}'.", DATA_DIRECTORY));

        info!("Created data directory at '{}'!", DATA_DIRECTORY);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_test() {
        let mut db = Database::new();

        db.capture("hello".to_string(), vec![], 100, 200);
        db.capture("hello".to_string(), vec![], 300, 450);

        let name_str = "hello";

        let avg = db.average(name_str);
        assert_eq!(avg, Some(125.0));

        db.capture("hello".to_string(), vec![], 100, 300);
        db.capture("hello".to_string(), vec![], 300, 452);

        let avg = db.average(name_str).unwrap();
        assert_eq!(avg, 150.5);
    }

    #[test]
    fn concurrent_init_test() {
        let db = Database::new();

        // Start the background consumer - this returns immediately
        db.init();

        // You can now do other work while consumer runs in background
        println!("Consumer started in background");

        // If you need the handle for testing, use init_with_handle instead:
        let _handle = db.init_with_handle();
        // _handle.join().unwrap(); // This would block until the thread finishes
    }
}
