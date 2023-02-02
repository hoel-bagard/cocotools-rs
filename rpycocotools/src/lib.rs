extern crate cocotools;
use crate::cocotools::errors::MissingIdError;
use pyo3::exceptions::PyKeyError;
use std::path::{Path, PathBuf};
pub mod coco;
pub mod errors;
use errors::PyMissingIdError;
use pyo3::prelude::*;
use pyo3::types::PyUnicode;

#[pyfunction]
pub fn visualize_img(
    dataset: &coco::COCO,
    image_folder: &PyUnicode,
    img_id: u32,
) -> Result<(), PyMissingIdError> {
    let image_folder = image_folder.to_str().unwrap().to_owned();

    let anns = dataset
        .dataset
        .get_img_anns(img_id)
        .map_err(|err| MissingIdError::from(err))?;
    let img_name = &dataset
        .dataset
        .get_img(img_id)
        .map_err(|err| MissingIdError::from(err))?
        .file_name;
    let img_path = Path::new(&image_folder).join(img_name);

    cocotools::visualize::show_anns(img_path, anns, true)?;
    Ok(())
}

#[pymodule]
fn rpycocotools(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<coco::COCO>()?;
    m.add_function(wrap_pyfunction!(visualize_img, m)?)?;
    Ok(())
}
