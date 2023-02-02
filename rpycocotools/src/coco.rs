extern crate cocotools;
use pyo3::exceptions::PyKeyError;

use crate::cocotools::annotations::coco_types::{
    self, Annotation, Category, Dataset, Image, Segmentation,
};
use crate::cocotools::annotations::load_coco::HashmapDataset;
use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;

use pyo3::prelude::*;
use pyo3::types::PyUnicode;

#[pyclass]
#[derive(Debug, Clone)]
struct PyCategory(Category);

#[pymethods]
impl PyCategory {
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
}

impl From<Category> for PyCategory {
    fn from(cat: Category) -> Self {
        Self(cat)
    }
}

#[pyclass]
#[derive(Debug)]
pub struct COCO {
    // TODO: Redo COCO the same way PyCategory is done, as a wrapper around the rust crate version.
    //       Otherwise it's missing things like get_img_anns, etc...
    pub dataset: HashmapDataset,
    // #[pyo3(get)]
    // cats: HashMap<u32, Py<PyCategory>>,
}

#[pymethods]
impl COCO {
    #[new]
    fn new(_py: Python<'_>, annotations_path: &PyUnicode) -> PyResult<Self> {
        let annotations_path = annotations_path.to_str().unwrap().to_owned();

        let annotations_file_content =
            fs::read_to_string(annotations_path).unwrap_or_else(|error| {
                if error.kind() == ErrorKind::NotFound {
                    panic!("Could not find the annotations file: {:?}", error);
                } else {
                    panic!("Problem opening the annotations file: {:?}", error);
                }
            });

        let dataset: Dataset =
            serde_json::from_str(&annotations_file_content).expect("Error decoding the json file");

        let dataset = HashmapDataset::new(dataset).unwrap_or_else(|error| {
            panic!(
                "Found an annotation for an image id not in the dataset when creating the dataset: {:?}",
                error
            );
        });

        Ok(Self { dataset })
    }

    #[getter]
    fn cats(&self) -> PyResult<HashMap<u32, Py<PyCategory>>> {
        let mut py_cats: HashMap<u32, Py<PyCategory>> = HashMap::new();
        Python::with_gil(|py| {
            for (id, cat) in self.dataset.cats.clone().into_iter() {
                py_cats.insert(id, Py::new(py, PyCategory(cat)).unwrap());
            }
        });
        Ok(py_cats)
    }
}
