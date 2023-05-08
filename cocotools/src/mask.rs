//! Module containing utilities to convert data formats.
use ndarray::Array2;

pub mod conversions;

/// A boolean mask indicating for each pixel whether it belongs to the object or not.
pub type Mask = Array2<u8>;
