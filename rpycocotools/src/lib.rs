extern crate cocotools;
use pyo3::prelude::*;

pub mod coco;
pub mod errors;

#[pymodule]
fn rpycocotools(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<coco::PyCOCO>()?;
    m.add_class::<coco::PyCategory>()?;
    // m.add_function(wrap_pyfunction!(visualize_img, m)?)?;
    Ok(())
}
