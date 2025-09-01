use super::capture::{Capture, Epoch};
use super::queue::KQueue;
use log::info;
use pyo3::prelude::*;
use std::collections::VecDeque;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

// How many logs need to be in the queue before we write to the database
// Make configurable: https://github.com/JakeRoggenbuck/kronicler/issues/18
const DB_WRITE_BUFFER_SIZE: usize = 20;

#[pyclass]
pub struct Database {
    queue: KQueue,
    tx: Sender<Box<dyn FnOnce() + Send + 'static>>,
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

        Database {
            queue: KQueue::new(),
            tx,
        }
    }

    pub fn capture(&self, name: String, args: Vec<PyObject>, start: Epoch, end: Epoch) {
        self.queue.capture(name, args, start, end);

        let queue_clone = Arc::clone(&self.queue.queue);

        // Invoke the concurrent consumer
        self.execute(move || {
            Database::consume_capture(queue_clone);
        });
    }
}

impl Database {
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.tx.send(Box::new(f)).unwrap();
    }

    fn consume_capture(queue: Arc<Mutex<VecDeque<Capture>>>) {
        let mut q = queue.lock().unwrap();

        if q.len() > DB_WRITE_BUFFER_SIZE {
            info!("Starting bulk write!");
            while !q.is_empty() {
                let a = q.pop_front();
                // TODO: Add capture to database
                // Use a bulk_write function
                info!("Writing {:?}", a);
            }
        }
    }
}
