use pyo3::prelude::*;
use std::collections::VecDeque;

type Epoch = u64;

struct Capture {
    start: Epoch,
    end: Epoch,
}

#[pyclass]
struct LFQueue {
    queue: VecDeque<Capture>,
}

#[pymethods]
impl LFQueue {
    pub fn capture(&mut self, start: Epoch, end: Epoch) {
        let c = Capture { start, end };
        self.queue.push_back(c);
    }

    #[new]
    pub fn new() -> Self {
        LFQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn logfrog(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LFQueue>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn add_to_lfq_test() {
        let mut lfq = LFQueue::new();

        let t1 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Not outatime.")
            .as_secs();

        let t2 = t1 + 100;

        // Check that it's empty before we add
        assert!(lfq.empty());

        lfq.capture(t1, t2);

        // Check that it's not empty after we add
        assert!(!lfq.empty());
    }
}
