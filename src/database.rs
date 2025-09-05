use super::bufferpool::Bufferpool;
use super::capture::Capture;
use super::column::Column;
use super::constants::{DATA_DIRECTORY, DB_WRITE_BUFFER_SIZE};
use super::queue::KQueue;
use super::row::{Epoch, FieldType, Row};
use log::info;
use pyo3::prelude::*;
use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};

#[pyclass]
pub struct Database {
    queue: KQueue,
    columns: Vec<Column>,
}

#[pymethods]
impl Database {
    #[new]
    pub fn new() -> Self {
        Database::check_for_data();
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

        let bp = Bufferpool::new(3);
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

        Database {
            queue: KQueue::new(),
            columns: vec![name_col, start_col, end_col],
        }
    }

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
}

impl Database {
    fn consume_capture(&mut self, queue: Arc<Mutex<VecDeque<Capture>>>) {
        let mut q = queue.lock().unwrap();

        if q.len() > DB_WRITE_BUFFER_SIZE {
            info!("Starting bulk write!");

            while !q.is_empty() {
                let capture = q.pop_front();

                let index = 0;

                if let Some(c) = capture {
                    // TODO: Replace for real ID
                    // Maybe it does not need an ID?
                    // Because the columns keep track of that
                    let row = c.to_row(index);

                    info!("Writing {:?}...", &row);

                    // Insert each field into its respective column
                    let mut col_index = 0;
                    for field in &row.fields {
                        self.columns[col_index].insert(field);
                        col_index += 1;
                    }
                }
            }
        }
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

    fn create_data_dir() {
        fs::create_dir_all(DATA_DIRECTORY)
            .expect(&format!("Could not create directory '{}'.", DATA_DIRECTORY));

        info!("Created data directory at '{}'!", DATA_DIRECTORY);
    }
}
