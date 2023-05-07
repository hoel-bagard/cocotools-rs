//! Module containing the structs used to build a COCO format dataset.
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::errors::{self, LoadingError, MissingIdError};
use crate::utils::load_img;
use crate::visualize::draw;

/// COCO dataset as-is, without additionnal functionalities.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Dataset {
    pub images: Vec<Image>,
    pub annotations: Vec<Annotation>,
    pub categories: Vec<Category>,
}

/// Stores information relating to one image.
#[cfg_attr(feature = "pyo3", pyclass(get_all, module = "rpycocotools.anns"))]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Image {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub file_name: String,
    // "license": int,
    // "flickr_url": str,
    // "coco_url": str,
    // "date_captured": datetime,
}

/// Object instance annotation for object detection.\
///
/// Each object instance annotation contains a series of fields, including the category id and segmentation mask of the object.\
/// In [the original COCO dataset](https://cocodataset.org/#home), the segmentation format depends on whether the instance represents a single object (`iscrowd=0` in which case polygons are used) or a collection of objects (`iscrowd=1` in which case RLE is used). Note that a single object (iscrowd=0) may require multiple polygons, for example if occluded.\
/// Crowd annotations (`iscrowd=1`) are used to label large groups of objects (e.g. a crowd of people). In addition, an enclosing bounding box is provided for each object (box coordinates are measured from the top left image corner and are 0-indexed).\
/// Finally, the categories field of the annotation structure stores the mapping of category id to category and supercategory names.
#[cfg_attr(feature = "pyo3", pyclass(get_all, module = "rpycocotools.anns"))]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Annotation {
    pub id: u32,
    pub image_id: u32,
    pub category_id: u32,
    /// Segmentation in the annotation file can be a polygon, RLE or COCO RLE.\
    /// Examples of what each segmentation should look like in the JSON file:
    /// - [`Polygons`]: `"segmentation": [[510.66, 423.01, 511.72, 420.03, ..., 510.45, 423.01]]`
    /// - [`Rle`]: `"segmentation": {"size": [40, 40], "counts": [245, 5, 35, 5, ..., 5, 35, 5, 1190]}`
    /// - [`CocoRle`]: `"segmentation": {"size": [480, 640], "counts": "aUh2b0X...BgRU4"}`
    pub segmentation: Segmentation,
    pub area: f64,
    /// The COCO bounding box format is `[top left x position, top left y position, width, height]`.\
    /// Example: "bbox": `[473.07, 395.93, 38.65, 28.67]`
    pub bbox: Bbox,
    /// Either 1 or 0
    pub iscrowd: u32,
}

// #[cfg_attr(feature = "pyo3", pyclass)]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Segmentation {
    Rle(Rle),
    CocoRle(CocoRle),
    Polygons(Polygons),
    #[serde(skip)]
    PolygonsRS(PolygonsRS),
}

/// Polygon(s) representing a segmentation mask.
///
/// A Segmentation mask might require multiple polygons if the mask is in multiple parts (in case of partial occlusion for example).
///
/// Each `Vec<f64>` represents an enclosed area belonging to the segmentation mask.
/// The length of each vector must be even. Every 2*n value represents the x coordinates of the nth point, while the 2*n+1 represents its y coordinates.
///
/// # Example:
/// ```rust
/// # use cocotools::coco::object_detection::Polygons;
/// let poly: Polygons = vec![vec![510.66, 423.01, 511.72, 420.03, 510.45, 423.01], vec![10.0, 10.0, 15.0, 15.0, 10.0, 15.0]];
/// assert_eq!(poly.len(), 2);
/// assert_eq!(poly[0].len() % 2, 0);
/// ```
pub type Polygons = Vec<Vec<f64>>;

/// Internal type used to represent polygons.
///
/// It contains the width and height of the image for easier handling, notably when using traits.
#[cfg_attr(feature = "pyo3", pyclass(get_all, module = "rpycocotools.anns"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PolygonsRS {
    /// Vector with two elements, the width and height of the image corresponding to the segmentation mask.
    pub size: Vec<u32>,
    /// See [`Polygons`].
    pub counts: Vec<Vec<f64>>,
}

/// Segmentation mask compressed as a [Run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding).
#[cfg_attr(
    feature = "pyo3",
    pyclass(get_all, name = "RLE", module = "rpycocotools.anns")
)]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Rle {
    /// Vector with two elements, the height and width of the image corresponding to the segmentation mask.
    pub size: Vec<u32>,
    pub counts: Vec<u32>,
}

/// Segmentation mask compressed as a [Run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding) and then further compressed into a string.
///
/// For the encoding process, see [here](https://github.com/cocodataset/cocoapi/blob/master/common/maskApi.c#L204).
#[cfg_attr(
    feature = "pyo3",
    pyclass(get_all, name = "COCO_RLE", module = "rpycocotools.anns")
)]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct CocoRle {
    /// Vector with two elements, the height and width of the image corresponding to the segmentation mask.
    pub size: Vec<u32>,
    pub counts: String,
}

/// Bounding box enclosing an object.
#[cfg_attr(
    feature = "pyo3",
    pyclass(sequence, get_all, name = "BBox", module = "rpycocotools.anns")
)]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Bbox {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

/// Category of an annotation.
#[cfg_attr(feature = "pyo3", pyclass(get_all, module = "rpycocotools.anns"))]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub supercategory: String,
}

/// COCO dataset represented as a hashmap where the hashmap's keys are the ids.
///
/// This struct provides methods to make working with the dataset easier and more efficient.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct HashmapDataset {
    pub(crate) anns: HashMap<u32, Annotation>,
    cats: HashMap<u32, Category>,
    imgs: HashMap<u32, Image>,
    /// Hashmap that links an image id to the image's annotations
    // Use Rc to reference the annotations directly ?
    img_to_anns: HashMap<u32, HashSet<u32>>,
    pub image_folder: PathBuf,
}

impl HashmapDataset {
    /// Loads a COCO dataset from the annotation file and the image folder.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the json file does not exist/cannot be read or if an error happens when deserializing and parsing it.
    /// Will return `Err` if there is an annotation with an image id X, but no image entry has this id.
    pub fn new<P: AsRef<Path>>(annotations_path: P, image_folder: P) -> Result<Self, LoadingError> {
        let annotations_path = annotations_path.as_ref().to_path_buf();
        let annotations_file_content = fs::read_to_string(&annotations_path)
            .map_err(|err| LoadingError::Read(err, annotations_path.clone()))?;

        let dataset: Dataset = serde_json::from_str(&annotations_file_content)
            .map_err(|err| LoadingError::Deserialize(err, annotations_path.clone()))?;

        let cats = dataset
            .categories
            .into_iter()
            .map(|category| (category.id, category))
            .collect();

        let imgs: HashMap<u32, Image> = dataset
            .images
            .clone()
            .into_iter()
            .map(|image| (image.id, image))
            .collect();

        let mut anns: HashMap<u32, Annotation> = HashMap::new();
        // Have (at least) an empty set for each image to avoid getting an error in the case where an image does not have any annotation.
        let mut img_to_anns: HashMap<u32, HashSet<u32>> = dataset
            .images
            .into_iter()
            .map(|image| (image.id, HashSet::new()))
            .collect();

        for mut annotation in dataset.annotations {
            let ann_id = annotation.id;
            let img_id = annotation.image_id;

            // The polygon format from COCO is annoying to deal with as it does not contain the size of the image,
            // it is therefore transformed into a more complete format.
            if let Segmentation::Polygons(counts) = annotation.segmentation {
                annotation.segmentation = Segmentation::PolygonsRS(PolygonsRS {
                    size: if let Some(img) = imgs.get(&img_id) {
                        vec![img.height, img.width]
                    } else {
                        return Err(MissingIdError::Image(img_id))
                            .map_err(|err| LoadingError::Parsing(err, annotations_path));
                    },
                    counts,
                });
            };

            anns.insert(annotation.id, annotation);
            img_to_anns
                .entry(img_id)
                .or_insert_with(HashSet::new)
                .insert(ann_id);
        }

        Ok(Self {
            anns,
            cats,
            imgs,
            img_to_anns,
            image_folder: image_folder.as_ref().to_path_buf(),
        })
    }

    /// Return the annotation corresponding to the given annotation id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is no entry in the dataset corresponding to `ann_id`.
    pub fn get_ann(&self, ann_id: u32) -> Result<&Annotation, MissingIdError> {
        self.anns
            .get(&ann_id)
            .ok_or(MissingIdError::Annotation(ann_id))
    }

    /// Returns all the annotations of the dataset.
    #[must_use]
    pub fn get_anns(&self) -> Vec<&Annotation> {
        self.anns.values().collect()
    }

    /// Return the category corresponding to the given category id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is no entry corresponding to `cat_id`.
    pub fn get_cat(&self, cat_id: u32) -> Result<&Category, MissingIdError> {
        self.cats
            .get(&cat_id)
            .ok_or(MissingIdError::Category(cat_id))
    }

    /// Returns all the categories of the dataset.
    #[must_use]
    pub fn get_cats(&self) -> Vec<&Category> {
        self.cats.values().collect()
    }

    /// Return the image entry corresponding to the given image id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is no entry corresponding to `img_id`.
    pub fn get_img(&self, img_id: u32) -> Result<&Image, MissingIdError> {
        self.imgs.get(&img_id).ok_or(MissingIdError::Image(img_id))
    }

    /// Returns all the images of the dataset.
    #[must_use]
    pub fn get_imgs(&self) -> Vec<&Image> {
        self.imgs.values().collect()
    }

    /// Return the annotations for the given image id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is no entry corresponding to `img_id`.
    pub fn get_img_anns(&self, img_id: u32) -> Result<Vec<&Annotation>, MissingIdError> {
        self.img_to_anns
            .get(&img_id)
            .map_or(Err(MissingIdError::Image(img_id)), |ann_ids| {
                ann_ids.iter().map(|ann_id| self.get_ann(*ann_id)).collect()
            })
    }

    /// Draw the annotations for the given image id on the image and return it.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is no image or annotation entry for `img_id`. Or if the segmentation annotations could not be decompressed.
    pub fn draw_img_anns(
        &self,
        img_id: u32,
        draw_bbox: bool,
    ) -> Result<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, errors::CocoError> {
        let img_path = self.image_folder.join(&self.get_img(img_id)?.file_name);
        let mut img = load_img(&img_path)?;
        draw::anns(&mut img, &self.get_img_anns(img_id)?, draw_bbox)?;
        Ok(img)
    }

    /// Draw the annotation on the image and return it.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is no image for the annotation. Or if the segmentation annotations could not be decompressed.
    pub fn draw_ann(
        &self,
        ann: &Annotation,
        draw_bbox: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let img_path = self
            .image_folder
            .join(&self.get_img(ann.image_id)?.file_name);
        let mut img = load_img(&img_path)?;
        draw::anns(&mut img, &vec![ann], draw_bbox)?;
        Ok(())
    }

    /// Save the dataset to the given path.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    ///   - The file cannot be created (if the full directory path does not exist for example).
    ///   - The implementation of `Serialize` fails or the dataset contains non-string keys.
    pub fn save_to<P: AsRef<Path>>(&self, output_path: P) -> Result<(), Box<dyn Error>> {
        let dataset = Dataset::from(self);
        let f = fs::File::create(output_path)?;
        serde_json::to_writer_pretty(&f, &dataset)?;

        Ok(())
    }
}

impl From<&HashmapDataset> for Dataset {
    fn from(dataset: &HashmapDataset) -> Self {
        Self {
            images: dataset.get_imgs().into_iter().cloned().collect(),
            annotations: dataset.get_anns().into_iter().cloned().collect(),
            categories: dataset.get_cats().into_iter().cloned().collect(),
        }
    }
}

impl PartialEq for PolygonsRS {
    // TODO: redo this function in a clearer way:
    // - Search for the first point of self in other. If it's not there, then return false.
    // - Look left an right of other for the second point of self to know in which direction to rotate (if not there return false).
    // - Match elements one by one.
    fn eq(&self, other: &Self) -> bool {
        // Assume that there are no duplicated polygons within an annotation.
        if self.size != other.size || self.counts.len() != other.counts.len() {
            return false;
        }
        let other_polygons = other.counts.clone();
        for self_poly in &self.counts {
            let mut found_match = false;
            'outer: for other_poly in &other_polygons {
                let mut other_poly = other_poly.clone();
                if self_poly.len() != other_poly.len() {
                    continue;
                }
                for _ in 0..other_poly.len() {
                    if &other_poly == self_poly {
                        found_match = true;
                        break 'outer;
                    }
                    other_poly.rotate_right(1);
                }

                other_poly.reverse();

                let mut reversed_other_poly: Vec<f64> = Vec::new();
                for i in (0..other_poly.len()).step_by(2) {
                    reversed_other_poly.push(other_poly[i + 1]);
                    reversed_other_poly.push(other_poly[i]);
                }
                for _ in 0..reversed_other_poly.len() {
                    if &reversed_other_poly == self_poly {
                        found_match = true;
                        break 'outer;
                    }
                    reversed_other_poly.rotate_right(1);
                }
            }
            if !found_match {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::PolygonsRS;
    use rstest::rstest;

    #[rstest]
    #[case::single_polygon(
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![1.1, 2.1, 3.2, 4.2, 5.3, 6.3]] },
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![1.1, 2.1, 3.2, 4.2, 5.3, 6.3]] },
    )]
    #[case::two_polygons(
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![1.1, 2.1, 3.2, 4.2, 5.3, 6.3], vec![7.4, 8.4, 9.5, 10.5, 11.6, 12.6]]},
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![1.1, 2.1, 3.2, 4.2, 5.3, 6.3], vec![7.4, 8.4, 9.5, 10.5, 11.6, 12.6]]},
    )]
    #[case::two_polygons_different_order(
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![1.1, 2.1, 3.2, 4.2, 5.3, 6.3], vec![7.4, 8.4, 9.5, 10.5, 11.6, 12.6]]},
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![7.4, 8.4, 9.5, 10.5, 11.6, 12.6], vec![1.1, 2.1, 3.2, 4.2, 5.3, 6.3]]},
    )]
    #[case::different_start_point(
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![1.1, 2.1, 3.2, 4.2, 5.3, 6.3], vec![7.4, 8.4, 9.5, 10.5, 11.6, 12.6]]},
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![11.6, 12.6, 7.4, 8.4, 9.5, 10.5], vec![3.2, 4.2, 5.3, 6.3, 1.1, 2.1]]},
    )]
    #[case::reversed_order(
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![1.1, 2.1, 3.2, 4.2, 5.3, 6.3, 7.4, 8.4]]},
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![7.4, 8.4, 5.3, 6.3, 3.2, 4.2, 1.1, 2.1]]},
    )]
    fn polygon_equality(#[case] poly1: &PolygonsRS, #[case] poly2: &PolygonsRS) {
        assert_eq!(poly1, poly2);
    }

    #[rstest]
    #[case::different_length(
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![1.1, 2.1, 3.2, 4.2, 5.3, 6.3, 7.4, 8.4]]},
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![1.1, 2.1, 3.2, 4.2, 5.3, 6.3]]},
    )]
    #[case::different_digit(
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![1.1, 2.1, 3.2, 4.2, 5.3, 6.3]]},
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![2.1, 2.1, 3.2, 4.2, 5.3, 6.3]]},
    )]
    #[case::different_number_of_polygons(
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![1.1, 2.1, 3.2, 4.2, 5.3, 6.3, 7.4, 8.4]]},
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![1.1, 2.1, 3.2, 4.2, 5.3, 6.3, 7.4, 8.4], vec![7.4, 8.4, 9.5, 10.5, 11.6, 12.6]]},
    )]
    #[case::x_y_inverted(
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![1.1, 2.1, 3.2, 4.2, 5.3, 6.3]]},
        &PolygonsRS {size: vec![20, 20], counts: vec![vec![2.1, 2.1, 4.2, 3.2, 6.3, 5.3]]},
    )]
    fn polygon_inequality(#[case] poly1: &PolygonsRS, #[case] poly2: &PolygonsRS) {
        assert_ne!(poly1, poly2);
    }
}
