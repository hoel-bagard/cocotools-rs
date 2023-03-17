use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;

use crate::cocotools::errors::LoadingError;
use crate::cocotools::errors::MissingIdError;

pub struct PyLoadingError(LoadingError);

impl From<LoadingError> for PyLoadingError {
    fn from(error: LoadingError) -> Self {
        Self(error)
    }
}

impl From<PyLoadingError> for PyErr {
    fn from(error: PyLoadingError) -> Self {
        PyValueError::new_err(error.0.to_string())
    }
}

pub struct PyMissingIdError(MissingIdError);

impl From<MissingIdError> for PyMissingIdError {
    fn from(error: MissingIdError) -> Self {
        Self(error)
    }
}

impl From<PyMissingIdError> for PyErr {
    fn from(error: PyMissingIdError) -> Self {
        PyKeyError::new_err(error.0.to_string())
    }
}
