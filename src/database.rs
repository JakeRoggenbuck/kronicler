use super::bufferpool::Bufferpool;
use super::capture::Capture;
use super::column::Column;
use super::constants::{DATA_DIRECTORY, DB_WRITE_BUFFER_SIZE};
use super::queue::KQueue;
use super::row::Epoch;
use super::row::FieldType;
use log::info;
use pyo3::prelude::*;
use std::collections::VecDeque;
use std::fs;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

#[pyclass]
pub struct Database {
    queue: KQueue,
    tx: Sender<Box<dyn FnOnce() + Send + 'static>>,
    columns: Vec<Column>,
    bufferpool: Arc<RwLock<Bufferpool>>,
}

#[pymethods]
impl Database {
    #[new]
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Box<dyn FnOnce() + Send>>();

        thread::spawn(move || {
            while let Ok(task) = rx.recv() {
                task();
            }
        });

        Database::create_data_dir();

        let bp = Bufferpool::new();
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
            1,
            bufferpool.clone(),
            FieldType::Epoch(0),
        );

        Database {
            queue: KQueue::new(),
            bufferpool: bufferpool.clone(),
            tx,
            columns: vec![name_col, start_col, end_col],
        }
    }

    pub fn capture(&self, name: String, args: Vec<PyObject>, start: Epoch, end: Epoch) {
        self.queue.capture(name, args, start, end);

        // let queue_clone = Arc::clone(&self.queue.queue);

        // // Invoke the concurrent consumer
        // self.execute(move || {
        //     Database::consume_capture(queue_clone);
        // });
        //
        // TODO: Figure out how to call consume_capture
        // Maybe it makes sense to run it infinitely in the main loop
        // to check for captures added to the queue
    }
}

impl Database {
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.tx.send(Box::new(f)).unwrap();
    }

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

                    // Insert each field into its respective column
                    let mut col_index = 0;
                    for field in &row.fields {
                        self.columns[col_index].insert(field);

                        col_index += 1;
                    }

                    // TODO: Add capture to database
                    // Use a bulk_write function
                    // TODO: Now we can use self.<db_method> to write to database!
                    info!("Writing {:?}", &row);
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
