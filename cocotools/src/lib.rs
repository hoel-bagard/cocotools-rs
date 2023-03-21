//! The `cocotools` crate provides tools to load, manipulate, convert and visualize COCO format datasets.

//! The base class is the **[`COCO`]** struct, usually loaded from a COCO json annotation file.
//!
//! This crate aims to provide similar functionalities to the [python pycocotools package](https://pypi.org/project/pycocotools/) / [cocoapi](https://github.com/cocodataset/cocoapi) with additionnal utilities such as conversion between dataset formats. It also aims to have a better documentation and a more readable implementation.
//!
//! ## Usage example
//!
//! ```
//! # use std::path::PathBuf;
//! use cocotools::COCO;
//!
//! let annotations_file_path = PathBuf::from("../data_samples/coco_25k/annotations.json");
//! let image_folder_path = PathBuf::from("../data_samples/coco_25k/images");
//! let dataset = COCO::new(&annotations_file_path, &image_folder_path)?;
//! assert_eq!(dataset.get_img(17627)?.file_name, "000000017627.jpg");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod annotations;
pub mod argparse;
pub mod converters;
pub mod errors;
pub mod visualize;

#[doc(hidden)]
pub use crate::annotations::COCO;
