use database::Database;
use pyo3::prelude::*;
use row::Row;

pub mod bufferpool;
pub mod capture;
pub mod column;
pub mod constants;
pub mod database;
pub mod filewriter;
pub mod index;
pub mod metadata;
pub mod page;
pub mod queue;
pub mod row;

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
fn kronicler(m: &Bound<'_, PyModule>) -> PyResult<()> {
    init_logging();

    m.add_class::<Database>()?;
    m.add_class::<Row>()?;
    m.add_function(wrap_pyfunction!(database::database_init, m)?)?;
    Ok(())
}
