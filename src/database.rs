use super::bufferpool::Bufferpool;
use super::capture::Capture;
use super::column::Column;
use super::constants::{DATA_DIRECTORY, DB_WRITE_BUFFER_SIZE};
use super::index::Index;
use super::queue::KQueue;
use super::row::{create_function_name, Epoch, FieldType, Row};
use log::{debug, info, warn};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock, RwLock};
use std::thread;

pub struct DatabaseInner {
    queue: KQueue,
    columns: Vec<Column>,
    /// Index rows by `name` field in capture
    name_index: Index,
    /// Keep track of the current row being inserted
    row_id: AtomicUsize,
    sync_consume: bool,
}

impl DatabaseInner {
    fn new(sync_consume: bool) -> Self {
        if !sync_consume {
            eprintln!(
                "Async Consume not fully supported yet in v0.1.1. Please set `sync_consume=True`."
            );
            // std::process::exit(0);
        }

        Database::create_data_dir();
        Database::check_for_data();

        let column_count = 4;

        let bp = Bufferpool::new(column_count);
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

        let delta_col = Column::new(
            "delta".to_string(),
            3,
            bufferpool.clone(),
            FieldType::Epoch(0),
        );

        let columns = vec![name_col, start_col, end_col, delta_col];

        assert_eq!(columns.len(), column_count);

        for col in &columns {
            col.save();
        }

        let name_index = Index::new();

        DatabaseInner {
            queue: KQueue::new(),
            columns,
            name_index,
            row_id: AtomicUsize::new(0),
            sync_consume,
        }
    }

    fn consume_capture(&mut self, queue: Arc<RwLock<VecDeque<Capture>>>) {
        info!("Calling consume_capture");

        let mut q = queue.write().unwrap();

        if q.len() > DB_WRITE_BUFFER_SIZE {
            info!("Starting bulk write!");

            while !q.is_empty() {
                let capture = q.pop_front();

                if let Some(c) = capture {
                    // TODO: Replace for real ID
                    // Maybe it does not need an ID?
                    // Because the columns keep track of that

                    // Get the self.row_id value (prev) and then add one to self.row_id
                    let prev = self.row_id.fetch_add(1, Ordering::SeqCst);
                    let row = c.to_row(prev);

                    info!("Writing {:?}...", &row);

                    let mut col_index = 0;
                    for field in &row.fields {
                        self.columns[col_index].insert(field);
                        debug!("{:?}", self.name_index);
                        col_index += 1;
                    }

                    self.name_index.insert(row.clone(), 0);
                }
            }

            // Save columns if there was new data
            for col in &self.columns {
                col.save();
            }
        }
    }
}

// Separate singleton instances for sync and async modes
static DATABASE_SYNC: OnceLock<Arc<RwLock<DatabaseInner>>> = OnceLock::new();
static DATABASE_ASYNC: OnceLock<Arc<RwLock<DatabaseInner>>> = OnceLock::new();

// Shared queue state using atomic bools - one for each database type
static QUEUE_HAS_DATA_SYNC: AtomicBool = AtomicBool::new(false);
static QUEUE_HAS_DATA_ASYNC: AtomicBool = AtomicBool::new(false);

#[pyclass]
pub struct Database {
    sync_consume: bool,
}

impl Database {
    fn get_instance(&self) -> Arc<RwLock<DatabaseInner>> {
        if self.sync_consume {
            DATABASE_SYNC
                .get_or_init(|| {
                    info!("Creating sync DatabaseInner with sync_consume=true");
                    Arc::new(RwLock::new(DatabaseInner::new(true)))
                })
                .clone()
        } else {
            DATABASE_ASYNC
                .get_or_init(|| {
                    info!("Creating async DatabaseInner with sync_consume=false");
                    Arc::new(RwLock::new(DatabaseInner::new(false)))
                })
                .clone()
        }
    }

    fn get_queue_state(&self) -> &'static AtomicBool {
        if self.sync_consume {
            &QUEUE_HAS_DATA_SYNC
        } else {
            &QUEUE_HAS_DATA_ASYNC
        }
    }

    fn create_data_dir() {
        fs::create_dir_all(DATA_DIRECTORY)
            .expect(&format!("Could not create directory '{}'.", DATA_DIRECTORY));

        info!("Created data directory at '{}'!", DATA_DIRECTORY);
    }

    fn check_for_data() {
        if !Database::exists() {
            eprintln!("Database does not exist at \"{}\".", &DATA_DIRECTORY);
            std::process::exit(0);
        }
    }
}

#[pymethods]
impl Database {
    #[new]
    #[pyo3(signature = (sync_consume = false))]
    pub fn new(sync_consume: bool) -> Self {
        info!("Creating Database with sync_consume={}", sync_consume);
        Database { sync_consume }
    }

    pub fn init(&mut self) {
        let db_instance = self.get_instance();
        let queue_state = self.get_queue_state();

        info!("Called init!");
        loop {
            // Use relaxed ordering since we don't need strict synchronization here
            if queue_state.load(Ordering::Relaxed) {
                info!("Running consume.");

                let queue_clone = {
                    let db = db_instance.read().unwrap();
                    Arc::clone(&db.queue.queue)
                };

                {
                    let mut db = db_instance.write().unwrap();
                    db.consume_capture(queue_clone);
                }

                // Mark queue as processed
                queue_state.store(false, Ordering::Relaxed);

                // let timeout = time::Duration::from_millis(CONSUMER_DELAY);
                // thread::sleep(timeout);
            }
        }
    }

    #[staticmethod]
    pub fn exists() -> bool {
        Path::new(&DATA_DIRECTORY).exists()
    }

    #[staticmethod]
    #[pyo3(signature = (sync_consume = false))]
    pub fn new_reader(sync_consume: bool) -> Self {
        info!(
            "Creating Database reader with sync_consume={}",
            sync_consume
        );
        Database::new(sync_consume)
    }

    /// Capture a function and write it to the queue
    pub fn capture(&mut self, name: String, args: Vec<PyObject>, start: Epoch, end: Epoch) {
        let db_instance = self.get_instance();
        let queue_state = self.get_queue_state();

        let mut db = db_instance.write().unwrap();

        // Signal that queue has new data
        queue_state.store(true, Ordering::Relaxed);

        info!("Capturing with sync_consume={}", db.sync_consume);
        db.queue.capture(name, args, start, end);

        if db.sync_consume {
            info!("Performing synchronous consume");
            let queue_clone = Arc::clone(&db.queue.queue);
            db.consume_capture(queue_clone);
            // For sync consume, we immediately mark as processed
            queue_state.store(false, Ordering::Relaxed);
        }
    }

    pub fn fetch(&mut self, index: usize) -> Option<Row> {
        info!("Starting fetch on index {}", index);

        let db_instance = self.get_instance();
        let mut db = db_instance.write().unwrap();
        let mut data = vec![];

        for col in &mut db.columns {
            let field = col.fetch(index);

            if let Some(f) = field {
                data.push(f);
            }
        }

        // TODO: Fix this to make it a better check for unwritten data
        if data.len() > 1 && data[1] == FieldType::Epoch(0) {
            return None;
        }

        Some(Row {
            id: index,
            fields: data,
        })
    }

    pub fn fetch_all(&mut self) -> Vec<Row> {
        let mut all = vec![];
        let mut index = 0;

        loop {
            let row = self.fetch(index);

            if let Some(r) = row {
                all.push(r);
            } else {
                break;
            }

            index += 1;
        }

        all
    }

    pub fn fetch_all_as_dict<'py>(&mut self, py: Python<'py>) -> Vec<Bound<'py, PyDict>> {
        let a = self.fetch_all().into_iter().map(|x| x.__dict__(py));
        let rows_dict: Vec<Bound<'py, PyDict>> = a.collect();
        rows_dict
    }

    pub fn logs<'py>(&mut self, py: Python<'py>) -> Vec<Bound<'py, PyDict>> {
        self.fetch_all_as_dict(py)
    }

    pub fn get_function_names(&mut self) -> HashSet<String> {
        let db_instance = self.get_instance();

        let db = db_instance.read().unwrap();

        let mut function_names = HashSet::new();
        let keys: Vec<&FieldType> = db.name_index.index.keys().into_iter().collect();

        for key in keys {
            function_names.insert(key.to_string());
        }

        function_names
    }

    /// Find the average time a function took to run
    pub fn average(&mut self, function_name: &str) -> Option<f64> {
        let name_bytes = create_function_name(function_name);

        let db_instance = self.get_instance();

        let db = db_instance.read().unwrap();
        if let Some(avg) = db.name_index.get_average(FieldType::Name(name_bytes)) {
            info!("Using amortized const average!");
            return Some(avg);
        }

        info!("Manually calculating average!");

        // Get the IDs we need to fetch
        let ids = {
            let db = db_instance.read().unwrap();
            db.name_index.get(FieldType::Name(name_bytes))
        };

        if let Some(ids) = ids {
            let mut values = vec![];

            for id in &ids {
                if let Some(row) = self.fetch(*id) {
                    let delta_index = 3;

                    let fs = row.fields;
                    if fs.len() > delta_index {
                        let f = fs[delta_index].clone();

                        match f {
                            FieldType::Epoch(e) => values.push(e),
                            _ => {}
                        }
                    }
                }
            }

            if !values.is_empty() {
                let sum: u128 = values.iter().sum();
                let avg = sum as f64 / values.len() as f64;
                return Some(avg);
            }
        } else {
            warn!("Could not get IDs");
        }

        None
    }
}

#[pyfunction]
pub fn database_init() {
    thread::spawn(|| {
        let mut db = Database::new(false);
        db.init();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_test() {
        let mut db = Database::new(true);

        db.capture("hello".to_string(), vec![], 100, 200);
        db.capture("hello".to_string(), vec![], 300, 450);

        let name_str = "hello";

        let avg = db.average(name_str);
        assert_eq!(avg, Some(125.0));

        db.capture("hello".to_string(), vec![], 100, 300);
        db.capture("hello".to_string(), vec![], 300, 452);

        let avg = db.average(name_str);
        assert_eq!(avg, Some(150.5));
    }

    #[test]
    fn get_function_names() {
        let mut db = Database::new(true);

        db.capture("hello".to_string(), vec![], 100, 200);
        db.capture("hello".to_string(), vec![], 300, 450);

        db.capture("hey".to_string(), vec![], 300, 450);
        db.capture("a".to_string(), vec![], 300, 450);

        let mut r = HashSet::new();
        r.insert("hello".to_string());
        r.insert("hey".to_string());
        r.insert("a".to_string());

        assert_eq!(db.get_function_names(), r);
    }

    #[test]
    fn singleton_test() {
        let mut db1 = Database::new(true);
        let mut db2 = Database::new(true);

        // Data inserted through db1 should be visible through db2
        db1.capture("test".to_string(), vec![], 100, 200);

        // This should be able to fetch the data inserted by db1
        let row = db2.fetch(0);
        assert!(row.is_some());
    }

    #[test]
    fn sync_vs_async_test() {
        let mut sync_db = Database::new(true);
        let mut async_db = Database::new(false);

        // These should use different singleton instances
        sync_db.capture("sync_test".to_string(), vec![], 100, 200);
        async_db.capture("async_test".to_string(), vec![], 100, 200);

        // sync_db should have its data immediately available
        let sync_row = sync_db.fetch(0);
        assert!(sync_row.is_some());

        // async_db might not have data immediately available (depends on buffer size)
        // but they should be separate instances
    }

    #[test]
    fn shared_queue_state_test() {
        let mut db1 = Database::new(false);
        let db2 = Database::new(false);

        // Both instances should see the same queue state
        let queue_state = db1.get_queue_state();

        // Initially false
        assert!(!queue_state.load(Ordering::Relaxed));

        // After capture through db1, both should see true
        db1.capture("test".to_string(), vec![], 100, 200);
        assert!(queue_state.load(Ordering::Relaxed));

        // Setting through one instance affects the other
        queue_state.store(false, Ordering::Relaxed);
        assert!(!db2.get_queue_state().load(Ordering::Relaxed));
    }
}
