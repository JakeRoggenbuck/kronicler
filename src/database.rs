use super::bufferpool::Bufferpool;
use super::capture::Capture;
use super::column::Column;
use super::constants::{DATA_DIRECTORY, DB_WRITE_BUFFER_SIZE};
use super::index::Index;
use super::queue::KQueue;
use super::row::{Epoch, FieldType, Row};
use log::{debug, info};
use pyo3::prelude::*;
use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};

#[pyclass]
pub struct Database {
    queue: KQueue,
    columns: Vec<Column>,
    /// Index rows by `name` field in capture
    name_index: Index,
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

        let name_index = Index::new();

        Database {
            queue: KQueue::new(),
            columns,
            name_index,
            // TODO: Load this in from metadata
            row_id: AtomicUsize::new(0),
        }
    }

    /// Capture a function and write it to the queue
    pub fn capture(&mut self, name: String, args: Vec<PyObject>, start: Epoch, end: Epoch) {
        self.queue.capture(name, args, start, end);

        let queue_clone = Arc::clone(&self.queue.queue);

        // // Invoke the concurrent consumer
        // self.execute(move || {
        //     Database::consume_capture(queue_clone);
        // });
        //
        // TODO: Figure out how to call consume_capture
        // Maybe it makes sense to run it infinitely in the main loop
        // to check for captures added to the queue

        // Doing this single threaded right now
        self.consume_capture(queue_clone);
    }

    pub fn fetch(&mut self, index: usize) -> Option<Row> {
        info!("Starting fetch on index {}", index);

        let mut data = vec![];

        for col in &mut self.columns {
            let field = col.fetch(index);

            if let Some(f) = field {
                data.push(f);
            }
        }

        // TODO: Fix this to make it a better check for unwritten data
        if data[1] == FieldType::Epoch(0) {
            return None;
        }

        Some(Row {
            id: index,
            fields: data,
        })
    }

    /// Find the average time a function took to run
    pub fn average(&mut self, function_name: &str) -> Option<f64> {
        let mut name_bytes = [0u8; 64];
        let bytes = function_name.as_bytes();
        name_bytes[..bytes.len()].copy_from_slice(bytes);

        if let Some(ids) = self.name_index.get(FieldType::Name(name_bytes)) {
            let mut values = vec![];

            for id in &ids {
                // TODO: This could be optimized
                if let Some(row) = self.fetch(*id) {
                    let delta_index = 3;

                    let fs = row.fields;
                    let f = fs[delta_index].clone();

                    match f {
                        FieldType::Epoch(e) => values.push(e),
                        _ => {}
                    }
                }
            }

            let sum: u128 = values.iter().sum();
            let avg = sum as f64 / values.len() as f64;

            return Some(avg);
        }

        None
    }
}

impl Database {
    fn consume_capture(&mut self, queue: Arc<Mutex<VecDeque<Capture>>>) {
        let mut q = queue.lock().unwrap();

        if q.len() > DB_WRITE_BUFFER_SIZE {
            info!("Starting bulk write!");

            while !q.is_empty() {
                let capture = q.pop_front();

                if let Some(c) = capture {
                    // TODO: Replace for real ID
                    // Maybe it does not need an ID?
                    // Because the columns keep track of that

                    // Get the self.row_id value (prev) and then add one to self.row_id
                    let prev = self.row_id.fetch_add(1, Ordering::SeqCst);
                    let row = c.to_row(prev);

                    info!("Writing {:?}...", &row);

                    // Insert each field into its respective column
                    let mut col_index = 0;
                    for field in &row.fields {
                        self.columns[col_index].insert(field);

                        debug!("{:?}", self.name_index);

                        col_index += 1;
                    }

                    self.name_index.insert(row.clone(), 0);
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

        let avg = db.average(name_str).unwrap();
        assert_eq!(avg, 125.0);

        db.capture("hello".to_string(), vec![], 100, 300);
        db.capture("hello".to_string(), vec![], 300, 452);

        let avg = db.average(name_str).unwrap();
        assert_eq!(avg, 150.5);
    }
}
