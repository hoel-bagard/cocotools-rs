extern crate cocotools;
use pyo3::prelude::*;

pub mod coco;
pub mod errors;
pub mod mask;

#[pymodule]
fn rpycocotools(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<coco::PyCOCO>()?;
    m.add_class::<cocotools::annotations::coco::Annotation>()?;
    m.add_function(wrap_pyfunction!(mask::decode_poly, m)?)?;
    Ok(())
}
