use pyo3::class::basic::CompareOp;
use pyo3::prelude::*;

use crate::annotations::coco::*;

#[pymethods]
impl Annotation {
    fn __repr__(&self) -> String {
        format!(
            "Annotation(id={}, image_id={}, category_id={}, segmentation={}, area={}, bbox={}, iscrowd={})",
            self.id, self.image_id, self.category_id, &self.segmentation.__repr__(), self.area, &self.bbox.__repr__(), self.iscrowd
        )
    }
}

#[pymethods]
impl Category {
    #[new]
    fn new(id: u32, name: String, supercategory: String) -> Self {
        Self {
            id,
            name,
            supercategory,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Category(id={}, name='{}', supercategory='{}')",
            self.id, self.name, self.supercategory
        )
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.id == other.id
                && self.name == other.name
                && self.supercategory == other.supercategory)
                .into_py(py),
            CompareOp::Ne => (self.id != other.id
                || self.name != other.name
                || self.supercategory != other.supercategory)
                .into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

#[pymethods]
impl Bbox {
    fn __repr__(&self) -> String {
        format!(
            "Bbox(left={}, top={}, width={}, height={})",
            self.left, self.top, self.width, self.height
        )
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.left == other.left
                && self.top == other.top
                && self.width == other.width
                && self.height == other.height)
                .into_py(py),
            CompareOp::Ne => (self.left != other.left
                || self.top != other.top
                || self.width != other.width
                || self.height != other.height)
                .into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

#[pymethods]
impl Rle {
    fn __repr__(&self) -> String {
        format!("RLE(counts={:?}, size={:?})", self.counts, self.size)
    }
}

#[pymethods]
impl EncodedRle {
    fn __repr__(&self) -> String {
        format!("EncodedRLE(counts={:?}, size={:?})", self.counts, self.size)
    }
}

#[pymethods]
impl PolygonRS {
    fn __repr__(&self) -> String {
        format!("Polygon(counts={:?})", self.counts)
    }
}

impl Segmentation {
    fn __repr__(&self) -> String {
        match self {
            Segmentation::Rle(rle) => rle.__repr__(),
            Segmentation::EncodedRle(encoded_rle) => encoded_rle.__repr__(),
            Segmentation::Polygon(poly) => format!("Polygon(counts={:?})", poly),
            Segmentation::PolygonRS(poly) => poly.__repr__(),
        }
    }
}

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
