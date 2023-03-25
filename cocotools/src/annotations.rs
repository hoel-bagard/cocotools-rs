//! Module containing the data annotation formats.

pub mod coco;
#[cfg(feature = "pyo3")]
pub mod coco_pyo3;

pub use crate::annotations::coco::HashmapDataset as COCO;
