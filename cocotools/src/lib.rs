//! The `cocotools` crate provides tools to load, manipulate, convert and visualize COCO format datasets.

//! The base classe is the **[`COCO`]** struct, usually load from a COCO json annotation file.
//!
//! This crate aims to provide similar functionalities to the [python pycocotools package](https://pypi.org/project/pycocotools/) / [cocoapi](https://github.com/cocodataset/cocoapi) with additionnal utilities such as conversion between dataset formats. It also aims to have a better documentation and a more readable implementation.
//!
//! ## Usage example
//!
//! ```
//!
//! let dataset = coco::load_anns(annotations_file)?;
//! ```
//!
//!
//!
//!
//!

pub mod annotations;
pub mod argparse;
pub mod converters;
pub mod errors;
pub mod visualize;

pub use crate::annotations::coco::HashmapDataset as COCO;
