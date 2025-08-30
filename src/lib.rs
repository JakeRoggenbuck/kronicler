use pyo3::prelude::*;
use std::collections::VecDeque;
use std::time::Instant;

struct Capture {
    start: Instant,
    end: Instant,
}

struct LFQueue {
    queue: VecDeque<Capture>,
}

impl LFQueue {
    pub fn capture(&mut self, start: Instant, end: Instant) {
        let c = Capture { start, end };
        self.queue.push_back(c);
    }

    pub fn new() -> Self {
        LFQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn logfrog(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn add_to_lfq_test() {
        let mut lfq = LFQueue::new();

        let t1 = Instant::now();
        let t2 = Instant::now() + Duration::from_millis(100);

        assert!(lfq.empty());

        lfq.capture(t1, t2);

        assert!(!lfq.empty());
    }
}
