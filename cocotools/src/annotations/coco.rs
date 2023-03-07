use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::errors::{LoadingError, MissingIdError};

#[derive(Deserialize, Serialize, Debug)]
pub struct Dataset {
    pub images: Vec<Image>,
    pub annotations: Vec<Annotation>,
    pub categories: Vec<Category>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Image {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub file_name: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Annotation {
    pub id: u32,
    pub image_id: u32,
    pub category_id: u32,
    /// Segmentation can be a polygon, RLE or encoded RLE.
    /// Exemple of polygon: "segmentation": [[510.66,423.01,511.72,420.03,...,510.45,423.01]]
    /// Exemple of RLE: "segmentation": {"size": [40, 40], "counts": [245, 5, 35, 5, 35, 5, 35, 5, 35, 5, 1190]}
    /// Exemple of encoded RLE: "segmentation": {"size": [480, 640], "counts": "aUh2b0X...BgRU4"}
    pub segmentation: Segmentation,
    pub area: f64,
    /// The COCO bounding box format is [top left x position, top left y position, width, height].
    /// bbox exemple:  "bbox": [473.07,395.93,38.65,28.67]
    pub bbox: Bbox,
    /// Either 1 or 0
    pub iscrowd: u32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Segmentation {
    Polygon(Polygon),
    #[serde(skip)]
    PolygonRS(PolygonRS),
    Rle(Rle),
    EncodedRle(EncodedRle),
}

pub type Polygon = Vec<Vec<f64>>;

/// Internal type used to represent a polygon. It contains the width and height of the image for easier handling, notably when using traits.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PolygonRS {
    pub size: Vec<u32>,
    pub counts: Vec<f64>,
}

/// Size is [height, width]
#[derive(Clone, Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct Rle {
    pub size: Vec<u32>,
    pub counts: Vec<u32>,
}

#[derive(Clone, Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct EncodedRle {
    pub size: Vec<u32>,
    pub counts: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Bbox {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub supercategory: String,
}

/// Transforms the COCO dataset into a hashmap version where the ids are keys.
#[derive(Debug)]
pub struct HashmapDataset {
    pub anns: HashMap<u32, Annotation>,
    pub cats: HashMap<u32, Category>,
    imgs: HashMap<u32, Image>,
    /// Hashmap that links an image id to the image's annotations
    // Use Rc to reference the annotations directly ?
    img_to_anns: HashMap<u32, Vec<u32>>,
}

impl HashmapDataset {
    /// Creates a `HashmapDataset` from a standard COCO one.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is an annotation with an image id X, but no image entry has this id.
    pub fn new(dataset: Dataset) -> Result<Self, MissingIdError> {
        let mut anns: HashMap<u32, Annotation> = HashMap::new();
        let mut img_to_anns: HashMap<u32, Vec<u32>> = HashMap::new();

        let cats = dataset
            .categories
            .into_iter()
            .map(|category| (category.id, category))
            .collect();

        let imgs: HashMap<u32, Image> = dataset
            .images
            .into_iter()
            .map(|image| (image.id, image))
            .collect();

        for mut annotation in dataset.annotations {
            let ann_id = annotation.id;
            let img_id = annotation.image_id;

            // The polygon format from COCO is annoying to deal with as it does not contain the size of the image,
            // it is therefore transformed into a more complete format.
            if let Segmentation::Polygon(mut counts) = annotation.segmentation {
                annotation.segmentation = Segmentation::PolygonRS(PolygonRS {
                    size: if let Some(img) = imgs.get(&img_id) {
                        vec![img.height, img.width]
                    } else {
                        return Err(MissingIdError::Image(img_id));
                    },
                    counts: counts.remove(0),
                });
            };

            anns.insert(annotation.id, annotation);
            img_to_anns
                .entry(img_id)
                .or_insert_with(Vec::new)
                .push(ann_id);
        }

        Ok(Self {
            anns,
            cats,
            imgs,
            img_to_anns,
        })
    }

    /// Return a result containing the annotation struct corresponding to the given id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is no entry in the dataset corresponding to `ann_id`.
    pub fn get_ann(&self, ann_id: u32) -> Result<&Annotation, MissingIdError> {
        self.anns
            .get(&ann_id)
            .ok_or(MissingIdError::Annotation(ann_id))
    }

    /// Overwrite if already present.
    pub fn add_ann(&mut self, ann: &Annotation) {
        self.anns.insert(ann.id, ann.clone());
        self.img_to_anns
            .entry(ann.image_id)
            .or_insert_with(Vec::new)
            .push(ann.id);
    }

    pub fn get_anns(&self) -> Vec<&Annotation> {
        self.anns.values().collect()
    }

    /// Return a result containing the category struct corresponding to the given id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is no entry corresponding to `cat_id`.
    pub fn get_cat(&self, cat_id: u32) -> Result<&Category, MissingIdError> {
        self.cats
            .get(&cat_id)
            .ok_or(MissingIdError::Category(cat_id))
    }
    pub fn get_cats(&self) -> Vec<&Category> {
        self.cats.values().collect()
    }

    /// Return a result containing the annotations for the given image id.
    ///

    /// Return a result containing the image struct corresponding to the given image id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is no entry corresponding to `img_id`.
    pub fn get_img(&self, img_id: u32) -> Result<&Image, MissingIdError> {
        self.imgs.get(&img_id).ok_or(MissingIdError::Image(img_id))
    }

    pub fn get_imgs(&self) -> Vec<&Image> {
        self.imgs.values().collect()
    }
    /// Return a result containing the annotations for the given image id.
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
}

impl From<HashmapDataset> for Dataset {
    fn from(dataset: HashmapDataset) -> Self {
        Self {
            images: dataset.get_imgs().into_iter().cloned().collect(),
            annotations: dataset.get_anns().into_iter().cloned().collect(),
            categories: dataset.get_cats().into_iter().cloned().collect(),
        }
    }
}

/// # Errors
///
/// Will return `Err` if the json file does not exist/cannot be read or if an error happens when deserializing and parsing it.
pub fn load_anns<P: AsRef<Path>>(annotations_path: P) -> Result<HashmapDataset, LoadingError> {
    let annotations_file_content = fs::read_to_string(&annotations_path)
        .map_err(|err| LoadingError::Read(err, annotations_path.as_ref().to_path_buf()))?;

    let dataset: Dataset = serde_json::from_str(&annotations_file_content)
        .map_err(|err| LoadingError::Deserialize(err, annotations_path.as_ref().to_path_buf()))?;

    HashmapDataset::new(dataset)
        .map_err(|err| LoadingError::Parsing(err, annotations_path.as_ref().to_path_buf()))
}

/// # Errors
///
/// Will return `Err` if:
///   - The file cannot be created (if the full directory path does not exist for example).
///   - The implementation of `Serialize` fails or the dataset contains non-string keys.
pub fn save_anns<P: AsRef<Path>>(
    _output_path: P,
    dataset: HashmapDataset,
) -> Result<(), Box<dyn Error>> {
    let dataset = Dataset::from(dataset);
    // let j = serde_json::to_string(&dataset);
    let f = fs::File::create("foo.json")?;
    serde_json::to_writer_pretty(&f, &dataset)?;

    Ok(())
}
