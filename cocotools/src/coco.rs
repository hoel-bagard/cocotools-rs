//! Module containing the data annotation formats.

pub mod object_detection;
#[cfg(feature = "pyo3")]
pub mod pyo3;

pub use crate::coco::object_detection::HashmapDataset as COCO;
