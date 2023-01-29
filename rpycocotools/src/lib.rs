extern crate cocotools;
pub mod coco;
use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rpycocotools(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<coco::COCO>()?;
    Ok(())
}
