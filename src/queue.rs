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

    #[test]
    fn queue_starts_empty() {
        let queue = KQueue::new();
        assert!(queue.empty());

        let q = queue.queue.read().unwrap();
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn multiple_captures_in_order() {
        let queue = KQueue::new();
        let t1 = 1000u128;
        let t2 = 2000u128;
        let t3 = 3000u128;

        queue.capture("first".to_string(), vec![], t1, t1 + 100);
        queue.capture("second".to_string(), vec![], t2, t2 + 200);
        queue.capture("third".to_string(), vec![], t3, t3 + 150);

        let q = queue.queue.read().unwrap();
        assert_eq!(q.len(), 3);

        // Verify captures are in order
        assert_eq!(q[0].name, "first");
        assert_eq!(q[0].delta, 100);
        assert_eq!(q[1].name, "second");
        assert_eq!(q[1].delta, 200);
        assert_eq!(q[2].name, "third");
        assert_eq!(q[2].delta, 150);
    }

    #[test]
    fn capture_calculates_delta_correctly() {
        let queue = KQueue::new();
        let start = 5000u128;
        let end = 7500u128;
        let expected_delta = 2500u128;

        queue.capture("test_function".to_string(), vec![], start, end);

        let q = queue.queue.read().unwrap();
        let capture = &q[0];

        assert_eq!(capture.start, start);
        assert_eq!(capture.end, end);
        assert_eq!(capture.delta, expected_delta);
    }

    #[test]
    fn concurrent_access_safety() {
        use std::thread;

        let queue = Arc::new(KQueue::new());
        let mut handles = vec![];

        // Spawn multiple threads adding captures concurrently
        for i in 0..10 {
            let q = Arc::clone(&queue);
            let handle = thread::spawn(move || {
                let start = (i * 1000) as u128;
                let end = start + 100;
                q.capture(format!("thread_{}", i), vec![], start, end);
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all captures were added
        let q = queue.queue.read().unwrap();
        assert_eq!(q.len(), 10);
        assert!(!queue.empty());
    }

    #[test]
    fn queue_preserves_capture_data() {
        let queue = KQueue::new();
        let name = "my_function".to_string();
        let start = 12345u128;
        let end = 67890u128;

        queue.capture(name.clone(), vec![], start, end);

        let q = queue.queue.read().unwrap();
        let capture = &q[0];

        assert_eq!(capture.name, name);
        assert_eq!(capture.start, start);
        assert_eq!(capture.end, end);
        assert_eq!(capture.args.len(), 0);
    }
}
