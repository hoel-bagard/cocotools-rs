use std::path::PathBuf;

use cocotools::annotations::coco;
use cocotools::errors::CocoError;
use cocotools::visualize::display;
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
        let annotations_path = PathBuf::from(annotations_path.to_str()?);
        let image_folder_path = PathBuf::from(image_folder_path.to_str()?);
        let dataset =
            COCO::new(annotations_path, image_folder_path).map_err(PyLoadingError::from)?;
        Ok(Self(dataset))
    }

    fn get_img(&self, py: Python<'_>, img_id: u32) -> PyResult<Py<coco::Image>> {
        Py::new(
            py,
            self.0
                .get_img(img_id)
                .map_err(PyMissingIdError::from)?
                .clone(),
        )
    }

    fn get_ann(&self, py: Python<'_>, ann_id: u32) -> PyResult<Py<coco::Annotation>> {
        Py::new(
            py,
            self.0
                .get_ann(ann_id)
                .map_err(PyMissingIdError::from)?
                .clone(),
        )
    }

    fn get_cat(&self, py: Python<'_>, cat_id: u32) -> PyResult<Py<coco::Category>> {
        Py::new(
            py,
            self.0
                .get_cat(cat_id)
                .map_err(PyMissingIdError::from)?
                .clone(),
        )
    }

    /// Order is non-deterministic
    fn get_imgs(&self, py: Python<'_>) -> PyResult<Vec<Py<coco::Image>>> {
        self.0
            .get_imgs()
            .into_iter()
            .map(|img| Py::new(py, img.clone()))
            .collect()
    }

    fn get_anns(&self, py: Python<'_>) -> PyResult<Vec<Py<coco::Annotation>>> {
        self.0
            .get_anns()
            .into_iter()
            .map(|ann| Py::new(py, ann.clone()))
            .collect()
    }

    fn get_cats(&self, py: Python<'_>) -> PyResult<Vec<Py<coco::Category>>> {
        self.0
            .get_cats()
            .into_iter()
            .map(|cat| Py::new(py, cat.clone()))
            .collect()
    }

    fn get_img_anns(&self, img_id: u32, py: Python<'_>) -> PyResult<Vec<Py<coco::Annotation>>> {
        self.0
            .get_img_anns(img_id)
            .map_err(PyMissingIdError::from)?
            .into_iter()
            .map(|ann| Py::new(py, ann.clone()))
            .collect()
    }

    /// Visualize an image and its annotations.
    ///
    /// ## Errors
    ///
    /// Will return `Err` if the image cannot be drawn (potentially due to it not being in the dataset) or cannot be displayed.
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

        display::img(&img, file_name)
            .map_err(|err| PyValueError::new_err(format!("Failed to display the image: {err}")))?;
        Ok(())
    }
}

#[pyclass(name = "Polygons", module = "rpycocotools.anns")]
#[derive(Debug)]
pub struct PyPolygons(cocotools::annotations::coco::Polygons);

#[pymethods]
impl PyPolygons {
    #[new]
    fn new(counts: Vec<Vec<f64>>) -> Self {
        Self(counts)
    }

    fn __repr__(&self) -> String {
        format!("Polygons(counts={:?})", self.0)
    }
}
