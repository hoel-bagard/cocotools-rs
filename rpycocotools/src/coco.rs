use std::collections::HashMap;
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

#[pyclass(name = "Annotation", module = "rpycocotools")]
#[derive(Debug, Clone)]
pub struct PyAnnotation(coco::Annotation);

#[pymethods]
impl PyAnnotation {
    // #[new]
    // fn new(
    //     id: u32,
    //     image_id: u32,
    //     category_id: u32,
    //     // segmentation: bool, // TODO
    //     area: f64,
    //     bbox: Vec<f64>,
    //     iscrow: u32,
    // ) -> Self {
    //     Self(coco::Annotation {
    //         id,
    //         image_id,
    //         category_id,
    //         segmentation,
    //         area,
    //         bbox,
    //         iscrowd,
    //     })
    // }

    #[getter]
    fn get_id(&self) -> u32 {
        self.0.id
    }

    #[getter]
    fn get_image_id(&self) -> u32 {
        self.0.image_id
    }

    #[getter]
    fn get_category_id(&self) -> u32 {
        self.0.category_id
    }

    // #[getter]
    // fn get_segmentation(&self) -> f64 {
    //     self.0.segmentation
    // }

    #[getter]
    fn get_area(&self) -> f64 {
        self.0.area
    }

    #[getter]
    fn get_bbox(&self) -> (f64, f64, f64, f64) {
        (
            self.0.bbox.left,
            self.0.bbox.top,
            self.0.bbox.width,
            self.0.bbox.height,
        )
    }

    #[getter]
    fn get_iscrowd(&self) -> u32 {
        self.0.iscrowd
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl From<coco::Annotation> for PyAnnotation {
    fn from(ann: coco::Annotation) -> Self {
        Self(ann)
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
        let dataset = COCO::new(annotations_path, image_folder_path)
            .map_err(PyLoadingError::from)?;
        Ok(Self(dataset))
    }

    #[getter]
    fn anns(&self) -> PyResult<HashMap<u32, Py<PyAnnotation>>> {
        let mut py_anns: HashMap<u32, Py<PyAnnotation>> = HashMap::new();
        Python::with_gil(|py| {
            for (id, ann) in self.0.anns.clone() {
                py_anns.insert(id, Py::new(py, PyAnnotation(ann)).unwrap());
            }
        });
        Ok(py_anns)
    }

    #[getter]
    fn cats(&self) -> PyResult<HashMap<u32, Py<PyCategory>>> {
        let mut py_cats: HashMap<u32, Py<PyCategory>> = HashMap::new();
        Python::with_gil(|py| {
            for (id, cat) in self.0.cats.clone() {
                py_cats.insert(id, Py::new(py, PyCategory(cat)).unwrap());
            }
        });
        Ok(py_cats)
    }

    #[setter(cats)]
    fn set_cats(&mut self, py_cats: HashMap<u32, Py<PyCategory>>) -> PyResult<()> {
        let mut cats: HashMap<u32, coco::Category> = HashMap::new();
        Python::with_gil(|py| {
            for (id, py_cat) in py_cats {
                cats.insert(id, py_cat.extract::<PyCategory>(py).unwrap().0);
            }
        });
        self.0.cats = cats;
        Ok(())
    }

    pub fn visualize_img(&self, img_id: u32) -> PyResult<()> {
        let img = self.0.draw_img_anns(img_id, true).unwrap();
        // .map_err(|err| PyValueError::new_err(err.to_string()))?;
        display_img(&img, &self.0.get_img(img_id).unwrap().file_name).unwrap();
        Ok(())
    }
}
