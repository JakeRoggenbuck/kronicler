use log::info;
use pyo3::prelude::*;
use std::collections::VecDeque;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

// How many logs need to be in the queue before we write to the database
const DB_WRITE_BUFFER_SIZE: usize = 100;

type Epoch = u128;

#[derive(Debug)]
struct Capture {
    name: String,
    args: Vec<PyObject>,
    start: Epoch,
    end: Epoch,
}

#[pyclass]
struct LFQueue {
    queue: Arc<Mutex<VecDeque<Capture>>>,
    tx: Sender<Box<dyn FnOnce() + Send + 'static>>,
}

// Internal Rust methods
impl LFQueue {
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

#[pymethods]
impl LFQueue {
    pub fn capture(&self, name: String, args: Vec<PyObject>, start: Epoch, end: Epoch) {
        let c = Capture {
            name,
            args,
            start,
            end,
        };

        info!("Added {:?} to log", &c);

        // Concurrently add the capture to the queue to be consumed later
        {
            let mut q = self.queue.lock().unwrap();
            q.push_back(c);
        }

        let queue_clone = Arc::clone(&self.queue);

        // Invoke the concurrent consumer
        self.execute(move || {
            LFQueue::consume_capture(queue_clone);
        });
    }

    #[new]
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Box<dyn FnOnce() + Send>>();

        thread::spawn(move || {
            while let Ok(task) = rx.recv() {
                task();
            }
        });

        LFQueue {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            tx,
        }
    }

    pub fn empty(&self) -> bool {
        self.queue.lock().unwrap().is_empty()
    }
}

/// Setup env logging
///
/// To use the logger, import the debug, error, or info macro from the log crate
///
/// Then you can add the macros to code like debug!("Start database!");
/// When you go to run the code, you can set the env var RUST_LOG=debug
/// Docs: https://docs.rs/env_logger/latest/env_logger/
#[inline]
fn init_logging() {
    let _ = env_logger::try_init();
}

/// A Python module implemented in Rust.
#[pymodule]
fn logfrog(m: &Bound<'_, PyModule>) -> PyResult<()> {
    init_logging();

    m.add_class::<LFQueue>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn add_to_lfq_test() {
        let lfq = LFQueue::new();

        let t1 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Not outatime.")
            .as_nanos();

        let t2 = t1 + 100;

        // Check that it's empty before we add
        assert!(lfq.empty());

        lfq.capture("foo".to_string(), vec![], t1, t2);

        // Check that it's not empty after we add
        assert!(!lfq.empty());
    }
}
