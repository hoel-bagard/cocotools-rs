use crate::annotations::coco_types::{self, Annotation, Category, Dataset, Image, Segmentation};
use crate::errors::MissingIdError;
use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

/// Transforms the COCO dataset into a hashmap version where the ids are keys.
#[derive(Debug)]
pub struct HashmapDataset {
    anns: HashMap<u32, Annotation>,
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
                annotation.segmentation = Segmentation::PolygonRS(coco_types::PolygonRS {
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

    /// Return a result containing the image struct corresponding to the given image id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is no entry corresponding to `img_id`.
    pub fn get_img(&self, img_id: u32) -> Result<&Image, MissingIdError> {
        self.imgs.get(&img_id).ok_or(MissingIdError::Image(img_id))
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

/// # Panics
///
/// Will panic if the json file does not exists, cannot be opened or if an error happens when creating a Hashmap version of it.
#[must_use]
pub fn load_json<P: AsRef<Path>>(annotations_path: P) -> HashmapDataset {
    let annotations_file_content = fs::read_to_string(annotations_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            panic!("Could not find the annotations file: {:?}", error);
        } else {
            panic!("Problem opening the annotations file: {:?}", error);
        }
    });

    let dataset: Dataset =
        serde_json::from_str(&annotations_file_content).expect("Error decoding the json file");

    HashmapDataset::new(dataset).unwrap_or_else(|error| {
        panic!(
            "Found an annotation for an image id not in the dataset when creating the dataset: {:?}",
            error
        );
    })
}
