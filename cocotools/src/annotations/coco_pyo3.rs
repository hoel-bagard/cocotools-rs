use pyo3::prelude::*;

use crate::annotations::coco::*;

#[pymethods]
impl Bbox {
    fn __repr__(&self) -> String {
        format!(
            "Bbox(left={}, top={}, width={}, height={})",
            self.left, self.top, self.width, self.height
        )
    }
}

// #[pymethods]
// impl Segmentation {
//     fn __repr__(&self) -> String {
//         match self {
//             Segmentation::Rle(rle) => "Segmentation of type rle",
//             Segmentation::EncodedRle(encoded_rle) => "Segmentation of type encoded_rle",
//             Segmentation::Polygon(poly) => "Segmentation of type poly",
//             Segmentation::PolygonRS(poly) => "Segmentation of type poly",
//         }
//     }
// }

impl IntoPy<PyObject> for Segmentation {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Segmentation::Rle(rle) => rle.into_py(py),
            Segmentation::EncodedRle(encoded_rle) => encoded_rle.into_py(py),
            Segmentation::Polygon(poly) => poly.into_py(py),
            Segmentation::PolygonRS(poly) => poly.into_py(py),
        }
    }
}
