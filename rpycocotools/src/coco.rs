use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use cocotools::annotations::coco;
use cocotools::COCO;
use pyo3::class::basic::CompareOp;
use pyo3::exceptions::PyKeyError;
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
pub struct PyCOCO {
    // TODO: Redo COCO the same way PyCategory is done, as a wrapper around the rust crate version.
    //       Otherwise it's missing things like get_img_anns, etc...
    pub dataset: COCO,
    // #[pyo3(get)]
    // cats: HashMap<u32, Py<PyCategory>>,
}

#[pymethods]
impl PyCOCO {
    #[new]
    fn new(_py: Python<'_>, annotations_path: &PyUnicode) -> PyResult<Self> {
        let annotations_path = PathBuf::from(annotations_path.to_str().unwrap());
        let dataset = COCO::try_from(&annotations_path).map_err(|err| PyLoadingError::from(err))?;
        Ok(Self { dataset })
    }

    #[getter]
    fn cats(&self) -> PyResult<HashMap<u32, Py<PyCategory>>> {
        // TODO: Try using a PyDict instead: https://docs.rs/pyo3/0.18.0/pyo3/types/struct.PyDict.html
        let mut py_cats: HashMap<u32, Py<PyCategory>> = HashMap::new();
        Python::with_gil(|py| {
            for (id, cat) in self.dataset.cats.clone().into_iter() {
                py_cats.insert(id, Py::new(py, PyCategory(cat)).unwrap());
            }
        });
        Ok(py_cats)
    }

    #[setter(cats)]
    fn set_cats(&mut self, py_cats: HashMap<u32, Py<PyCategory>>) -> PyResult<()> {
        let mut cats: HashMap<u32, coco::Category> = HashMap::new();
        Python::with_gil(|py| {
            for (id, py_cat) in py_cats.into_iter() {
                cats.insert(id, py_cat.extract::<PyCategory>(py).unwrap().0);
            }
        });
        self.dataset.cats = cats;
        Ok(())
    }
}
