use std::path::PathBuf;

use cocotools::annotations::coco;
use cocotools::errors::CocoError;
use cocotools::visualize::display::display_img;
use cocotools::COCO;
use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyUnicode;

use crate::errors::{PyLoadingError, PyMissingIdError};

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

    /// Order is non-deterministic
    fn get_imgs(&self, py: Python<'_>) -> Vec<Py<coco::Image>> {
        self.0
            .get_imgs()
            .into_iter()
            .map(|img| Py::new(py, img.clone()).unwrap())
            .collect()
    }

    fn get_anns(&self, py: Python<'_>) -> Vec<Py<coco::Annotation>> {
        self.0
            .get_anns()
            .into_iter()
            .map(|ann| Py::new(py, ann.clone()).unwrap())
            .collect()
    }

    fn get_cats(&self, py: Python<'_>) -> Vec<Py<coco::Category>> {
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
            .map_err(PyMissingIdError::from)?
            .into_iter()
            .map(|ann| Py::new(py, ann.clone()).unwrap())
            .collect())
    }

    pub fn visualize_img(&self, img_id: u32) -> PyResult<()> {
        let img = self
            .0
            .draw_img_anns(img_id, true)
            .map_err(|err| match err {
                CocoError::MissingId(err) => PyKeyError::new_err(err.to_string()),
                CocoError::Mask(err) => PyValueError::new_err(err.to_string()),
                CocoError::Loading(err) => PyValueError::new_err(err.to_string()),
            })?;

        let file_name = &self
            .0
            .get_img(img_id)
            .map_err(PyMissingIdError::from)?
            .file_name;

        display_img(&img, file_name).unwrap();
        Ok(())
    }
}
