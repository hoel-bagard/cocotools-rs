extern crate cocotools;
use pyo3::types::PyDict;
use pyo3::{prelude::*, wrap_pymodule};

pub mod coco;
pub mod errors;
pub mod mask;

#[pymodule]
fn anns(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_class::<cocotools::annotations::coco::Annotation>()?;
    module.add_class::<cocotools::annotations::coco::Bbox>()?;
    module.add_class::<cocotools::annotations::coco::Category>()?;
    module.add_class::<coco::PyPolygons>()?;
    module.add_class::<cocotools::annotations::coco::PolygonsRS>()?;
    module.add_class::<cocotools::annotations::coco::Rle>()?;
    module.add_class::<cocotools::annotations::coco::EncodedRle>()?;
    module.add_class::<cocotools::annotations::coco::Image>()?;
    Ok(())
}

#[pymodule]
fn rpycocotools(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_class::<coco::PyCOCO>()?;
    module.add_wrapped(wrap_pymodule!(anns))?;
    module.add_wrapped(wrap_pymodule!(mask::py_mask))?;

    // Inserting to sys.modules allows importing submodules nicely from Python
    // e.g. from rpycocotools.mask import decode_rle
    let sys = PyModule::import(py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("rpycocotools.mask", module.getattr("mask")?)?;
    sys_modules.set_item("rpycocotools.anns", module.getattr("anns")?)?;

    Ok(())
}
