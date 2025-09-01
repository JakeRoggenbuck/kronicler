use pyo3::prelude::*;

pub type Epoch = u128;

#[derive(Debug)]
pub struct Capture {
    pub name: String,
    pub args: Vec<PyObject>,
    pub start: Epoch,
    pub end: Epoch,
}
