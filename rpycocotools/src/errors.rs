use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;

use crate::cocotools::errors::MissingIdError;
use crate::cocotools::errors::MissingImageIdError;

// impl From<MissingImageIdError> for PyErr {
//     fn from(err: MissingImageIdError) -> Self {
//         // PyKeyError::new_err(format!(
//         //     "The following image id was not found in the dataset: {}",
//         //     err.id
//         // ));
//         PyKeyError::new_err(MissingIdError::Image(err).to_string());
//     }
// }

pub struct PyMissingIdError(MissingIdError);

impl From<PyMissingIdError> for PyErr {
    fn from(error: PyMissingIdError) -> Self {
        PyKeyError::new_err(error.0.to_string())
    }
}

impl From<MissingIdError> for PyMissingIdError {
    fn from(error: MissingIdError) -> Self {
        Self(error)
    }
}

pub struct PyMissingImageIdError(MissingImageIdError);

impl From<PyMissingImageIdError> for PyErr {
    fn from(error: PyMissingImageIdError) -> Self {
        PyKeyError::new_err(MissingIdError::Image(error.0).to_string())
    }
}

impl From<MissingImageIdError> for PyMissingImageIdError {
    fn from(error: MissingImageIdError) -> Self {
        Self(error)
    }
}
