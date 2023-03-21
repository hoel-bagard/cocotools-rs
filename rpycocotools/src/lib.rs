extern crate cocotools;
use pyo3::{prelude::*, wrap_pymodule};

pub mod coco;
pub mod errors;
pub mod mask;

#[pymodule]
fn anns(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_class::<cocotools::annotations::coco::Annotation>()?;
    module.add_class::<cocotools::annotations::coco::Bbox>()?;
    module.add_class::<cocotools::annotations::coco::Category>()?;
    module.add_class::<cocotools::annotations::coco::PolygonRS>()?;
    module.add_class::<cocotools::annotations::coco::Rle>()?;
    module.add_class::<cocotools::annotations::coco::EncodedRle>()?;
    module.add_class::<cocotools::annotations::coco::Image>()?;
    Ok(())
}

#[pymodule]
fn rpycocotools(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_class::<coco::PyCOCO>()?;
    module.add_wrapped(wrap_pymodule!(anns))?;
    module.add_wrapped(wrap_pymodule!(mask::mask))?;
    Ok(())
}
