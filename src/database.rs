use super::queue::{Epoch, KQueue};
use pyo3::prelude::*;

#[pyclass]
pub struct Database {
    queue: KQueue,
}

#[pymethods]
impl Database {
    #[new]
    pub fn new() -> Self {
        Database {
            queue: KQueue::new(),
        }
    }

    pub fn capture(&self, name: String, args: Vec<PyObject>, start: Epoch, end: Epoch) {
        self.queue.capture(name, args, start, end);
    }
}
