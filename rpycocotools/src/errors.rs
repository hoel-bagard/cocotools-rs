// TODO: Try to do a macro to avoid all the boiler plate.
// TODO: Expose the errors to the python api ? https://pyo3.rs/main/exception
use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;

use crate::cocotools::errors::{LoadingError, MaskError, MissingIdError};

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

pub struct PyMaskError(MaskError);

impl From<MaskError> for PyMaskError {
    fn from(error: MaskError) -> Self {
        Self(error)
    }
}

impl From<PyMaskError> for PyErr {
    fn from(error: PyMaskError) -> Self {
        PyValueError::new_err(error.0.to_string())
    }
}
