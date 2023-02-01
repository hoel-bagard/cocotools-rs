extern crate cocotools;
use pyo3::exceptions::PyKeyError;
use std::path::{Path, PathBuf};
pub mod coco;
use pyo3::prelude::*;
use pyo3::types::PyUnicode;

#[pyfunction]
pub fn visualize_img(dataset: &coco::COCO, image_folder: &PyUnicode, img_id: u32) -> PyResult<()> {
    let image_folder = image_folder.to_str().unwrap().to_owned();

    let anns = dataset.get_img_anns(img_id)?;
    let img_name = &dataset.get_img(img_id)?.file_name;
    let img_path = Path::new(image_folder).join(img_name);

    match cocotools::visualize::show_anns(img_path, anns, true) {
        Ok(_) => Ok(()),
        Err(e) => Err(PyKeyError::new_err(format!(
            "The following image id was not found in the dataset: {}",
            img_id
        ))),
    }
}

#[pymodule]
fn rpycocotools(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<coco::COCO>()?;
    m.add_function(wrap_pyfunction!(visualize_img, m)?)?;
    Ok(())
}
