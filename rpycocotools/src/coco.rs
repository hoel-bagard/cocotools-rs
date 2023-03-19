use std::path::PathBuf;

use cocotools::annotations::coco;
use cocotools::visualize::display::display_img;
use cocotools::COCO;
use pyo3::class::basic::CompareOp;
// use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyUnicode;

use crate::errors::PyLoadingError;

#[pyclass(name = "Category", module = "rpycocotools")]
#[derive(Debug, Clone)]
pub struct PyCategory(coco::Category);

#[pymethods]
impl PyCategory {
    #[new]
    fn new(id: u32, name: String, supercategory: String) -> Self {
        Self(coco::Category {
            id,
            name,
            supercategory,
        })
    }

    #[getter]
    fn id(&self) -> u32 {
        self.0.id
    }

    #[getter(name)]
    fn name(&self) -> String {
        self.0.name.clone()
    }

    #[setter(name)]
    fn set_name(&mut self, new_name: String) -> PyResult<()> {
        self.0.name = new_name;
        Ok(())
    }

    #[getter]
    fn supercategory(&self) -> String {
        self.0.supercategory.clone()
    }

    #[setter(supercategory)]
    fn set_supercategory(&mut self, supercategory: String) {
        self.0.supercategory = supercategory;
    }

    fn __repr__(&self) -> String {
        format!(
            "Category(id={}, name='{}', supercategory='{}')",
            self.0.id, self.0.name, self.0.supercategory
        )
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0.id == other.0.id
                && self.0.name == other.0.name
                && self.0.supercategory == other.0.supercategory)
                .into_py(py),
            CompareOp::Ne => (self.0.id != other.0.id
                || self.0.name != other.0.name
                || self.0.supercategory != other.0.supercategory)
                .into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

impl From<coco::Category> for PyCategory {
    fn from(cat: coco::Category) -> Self {
        Self(cat)
    }
}

#[pyclass(name = "COCO", module = "rpycocotools")]
#[derive(Debug)]
pub struct PyCOCO(COCO);

#[pymethods]
impl PyCOCO {
    #[new]
    fn new(
        _py: Python<'_>,
        annotations_path: &PyUnicode,
        image_folder_path: &PyUnicode,
    ) -> PyResult<Self> {
        let annotations_path = PathBuf::from(annotations_path.to_str().unwrap());
        let image_folder_path = PathBuf::from(image_folder_path.to_str().unwrap());
        let dataset =
            COCO::new(annotations_path, image_folder_path).map_err(PyLoadingError::from)?;
        Ok(Self(dataset))
    }

    #[getter]
    fn imgs(&self, py: Python<'_>) -> Vec<Py<coco::Image>> {
        self.0
            .get_imgs()
            .into_iter()
            .map(|img| Py::new(py, img.clone()).unwrap())
            .collect()
    }

    #[getter]
    fn anns(&self, py: Python<'_>) -> Vec<Py<coco::Annotation>> {
        self.0
            .get_anns()
            .into_iter()
            .map(|ann| Py::new(py, ann.clone()).unwrap())
            .collect()
    }

    #[getter]
    fn cats(&self, py: Python<'_>) -> Vec<Py<coco::Category>> {
        self.0
            .get_cats()
            .into_iter()
            .map(|cat| Py::new(py, cat.clone()).unwrap())
            .collect()
    }

    fn get_img_anns(&self, img_id: u32, py: Python<'_>) -> PyResult<Vec<Py<coco::Annotation>>> {
        Ok(self
            .0
            .get_img_anns(img_id)
            .unwrap()
            .into_iter()
            .map(|ann| Py::new(py, ann.clone()).unwrap())
            .collect())
    }

    pub fn visualize_img(&self, img_id: u32) -> PyResult<()> {
        let img = self.0.draw_img_anns(img_id, true).unwrap();
        // .map_err(|err| PyValueError::new_err(err.to_string()))?;
        display_img(&img, &self.0.get_img(img_id).unwrap().file_name).unwrap();
        Ok(())
    }
}
