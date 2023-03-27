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
impl Image {
    #[new]
    fn new(id: u32, width: u32, height: u32, file_name: String) -> Self {
        Self {
            id,
            width,
            height,
            file_name,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Image(id={}, width='{}', height='{}', file_name='{}')",
            self.id, self.width, self.height, self.file_name
        )
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.id == other.id
                && self.width == other.width
                && self.height == other.height
                && self.file_name == other.file_name)
                .into_py(py),
            CompareOp::Ne => (self.id != other.id
                || self.width != other.width
                || self.height != other.height
                || self.file_name != other.file_name)
                .into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

#[pyclass]
struct BboxIter {
    inner: std::vec::IntoIter<f64>,
}

#[pymethods]
impl BboxIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<f64> {
        slf.inner.next()
    }
}
#[pymethods]
impl Bbox {
    #[new]
    fn new(left: f64, top: f64, width: f64, height: f64) -> Self {
        Self {
            left,
            top,
            width,
            height,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Bbox(left={}, top={}, width={}, height={})",
            self.left, self.top, self.width, self.height
        )
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<BboxIter>> {
        let iter = BboxIter {
            inner: vec![slf.left, slf.top, slf.width, slf.height].into_iter(),
        };
        Py::new(slf.py(), iter)
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
    #[new]
    fn new(size: Vec<u32>, counts: Vec<u32>) -> Self {
        Self { size, counts }
    }

    fn __repr__(&self) -> String {
        format!("RLE(counts={:?}, size={:?})", self.counts, self.size)
    }
}

#[pymethods]
impl EncodedRle {
    #[new]
    fn new(size: Vec<u32>, counts: String) -> Self {
        Self { size, counts }
    }

    fn __repr__(&self) -> String {
        format!("EncodedRLE(counts={:?}, size={:?})", self.counts, self.size)
    }
}

#[pymethods]
impl PolygonsRS {
    #[new]
    fn new(size: Vec<u32>, counts: Vec<Vec<f64>>) -> Self {
        Self { size, counts }
    }

    fn __repr__(&self) -> String {
        format!("PolygonsRS(counts={:?})", self.counts)
    }
}

impl Segmentation {
    fn __repr__(&self) -> String {
        match self {
            Segmentation::Rle(rle) => rle.__repr__(),
            Segmentation::EncodedRle(encoded_rle) => encoded_rle.__repr__(),
            Segmentation::Polygons(poly) => format!("Polygons(counts={:?})", poly),
            Segmentation::PolygonsRS(poly) => poly.__repr__(),
        }
    }
}

impl IntoPy<PyObject> for Segmentation {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Segmentation::Rle(rle) => rle.into_py(py),
            Segmentation::EncodedRle(encoded_rle) => encoded_rle.into_py(py),
            Segmentation::Polygons(poly) => poly.into_py(py),
            Segmentation::PolygonsRS(poly) => poly.into_py(py),
        }
    }
}
