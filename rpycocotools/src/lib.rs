#![allow(clippy::redundant_pub_crate)]
extern crate cocotools;
use pyo3::types::PyDict;
use pyo3::{prelude::*, wrap_pymodule};

pub mod coco;
pub mod errors;
pub mod mask;

#[pymodule]
fn anns(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_class::<cocotools::coco::object_detection::Annotation>()?;
    module.add_class::<cocotools::coco::object_detection::Bbox>()?;
    module.add_class::<cocotools::coco::object_detection::Category>()?;
    module.add_class::<coco::PyPolygons>()?;
    module.add_class::<cocotools::coco::object_detection::PolygonsRS>()?;
    module.add_class::<cocotools::coco::object_detection::Rle>()?;
    module.add_class::<cocotools::coco::object_detection::CocoRle>()?;
    module.add_class::<cocotools::coco::object_detection::Image>()?;
    Ok(())
}

#[pymodule]
fn _rpycocotools(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_class::<coco::PyCOCO>()?;
    module.add_wrapped(wrap_pymodule!(anns))?;
    module.add_wrapped(wrap_pymodule!(mask::py_mask))?;

    // Inserting to sys.modules allows importing submodules nicely from Python
    // e.g. from rpycocotools.mask import decode_rle
    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("_rpycocotools.mask", module.getattr("mask")?)?;
    sys_modules.set_item("_rpycocotools.anns", module.getattr("anns")?)?;

    Ok(())
}
