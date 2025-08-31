use log::info;
use pyo3::prelude::*;
use std::collections::VecDeque;

type Epoch = u128;

#[derive(Debug, Copy, Clone)]
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

        info!("Added {:?} to log", c);
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
        let mut lfq = LFQueue::new();

        let t1 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Not outatime.")
            .as_nanos();

        let t2 = t1 + 100;

        // Check that it's empty before we add
        assert!(lfq.empty());

        lfq.capture(t1, t2);

        // Check that it's not empty after we add
        assert!(!lfq.empty());
    }
}
