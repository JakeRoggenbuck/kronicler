use super::capture::Capture;
use super::row::Epoch;
use log::info;
use pyo3::prelude::*;
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

#[pyclass]
pub struct KQueue {
    pub queue: Arc<RwLock<VecDeque<Capture>>>,
}

// Internal Rust methods
impl KQueue {}

#[pymethods]
impl KQueue {
    pub fn capture(&self, name: String, args: Vec<PyObject>, start: Epoch, end: Epoch) {
        let c = Capture {
            name,
            args,
            start,
            end,
            delta: end - start,
        };

        info!("Added {:?} to log", &c);

        // Concurrently add the capture to the queue to be consumed later
        {
            let mut q = self.queue.write().unwrap();
            q.push_back(c);
        }
    }

    #[new]
    pub fn new() -> Self {
        KQueue {
            queue: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    pub fn empty(&self) -> bool {
        self.queue.read().unwrap().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn add_to_lfq_test() {
        let lfq = KQueue::new();

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
