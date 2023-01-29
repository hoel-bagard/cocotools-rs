extern crate cocotools;
use pyo3::exceptions::PyKeyError;

use crate::cocotools::annotations::coco_types::{
    self, Annotation, Category, Dataset, Image, Segmentation,
};
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
    // TODO: https://pyo3.rs/main/faq.html#pyo3get-clones-my-field
    // #[setter(name)]
    // fn set_name(&mut self, new_name: String) -> PyResult<()> {
    //     self.0.name = new_name;
    //     Ok(())
    // }
    #[getter]
    fn supercategory(&self) -> String {
        self.0.supercategory.clone()
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Category(id={}, name='{}', supercategory='{}')",
            self.0.id, self.0.name, self.0.supercategory
        ))
    }
}

#[pyclass]
#[derive(Debug)]
pub struct COCO {
    anns: HashMap<u32, Annotation>,
    #[pyo3(get)]
    cats: HashMap<u32, PyCategory>,
    imgs: HashMap<u32, Image>,
    /// Hashmap that links an image id to the image's annotations
    img_to_anns: HashMap<u32, Vec<u32>>,
}

#[pymethods]
impl COCO {
    #[new]
    fn new(annotations_path: &PyUnicode) -> PyResult<Self> {
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

        let mut anns: HashMap<u32, Annotation> = HashMap::new();
        let mut cats: HashMap<u32, PyCategory> = HashMap::new();
        let mut imgs: HashMap<u32, Image> = HashMap::new();
        let mut img_to_anns: HashMap<u32, Vec<u32>> = HashMap::new();

        for category in dataset.categories {
            cats.insert(category.id, PyCategory(category));
        }

        for image in dataset.images {
            imgs.insert(image.id, image);
        }

        for mut annotation in dataset.annotations {
            let ann_id = annotation.id;
            let img_id = annotation.image_id;

            if let Segmentation::Polygon(mut counts) = annotation.segmentation {
                annotation.segmentation = Segmentation::PolygonRS(coco_types::PolygonRS {
                    size: if let Some(img) = imgs.get(&img_id) {
                        vec![img.height, img.width]
                    } else {
                        return Err(PyKeyError::new_err(format!(
                            "The following image id was not found in the dataset: {}",
                            img_id
                        )));
                    },
                    counts: counts.remove(0),
                });
            };

            anns.insert(annotation.id, annotation);
            img_to_anns.entry(img_id).or_insert_with(Vec::new);
            img_to_anns
                .get_mut(&img_id)
                .expect("Image id not in the hashmap, eventhough it should have been initialized on the previous line.")
                .push(ann_id);
        }

        Ok(Self {
            anns,
            cats,
            imgs,
            img_to_anns,
        })
    }

    // #[getter]
    // fn anns(&self) -> HashMap<u32, Annotation> {
    //     self.anns
    // }
}
