use crate::annotations::coco_types::{self, Annotation, Category, Dataset, Image, Segmentation};
use crate::errors;
use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;

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
    pub fn new(dataset: Dataset) -> Result<Self, errors::MissingImageIdError> {
        let mut anns: HashMap<u32, Annotation> = HashMap::new();
        let mut cats: HashMap<u32, Category> = HashMap::new();
        let mut imgs: HashMap<u32, Image> = HashMap::new();
        let mut img_to_anns: HashMap<u32, Vec<u32>> = HashMap::new();

        for category in dataset.categories {
            cats.insert(category.id, category);
        }

        for image in dataset.images {
            imgs.insert(image.id, image);
        }

        for mut annotation in dataset.annotations {
            let ann_id = annotation.id;
            let img_id = annotation.image_id;

            if let Segmentation::Polygon(mut counts) = annotation.segmentation {
                annotation.segmentation = Segmentation::PolygonRS(coco_types::PolygonRS {
                    size: if let Some(img) = imgs.get(&img_id) {
                        vec![img.height, img.width]
                    } else {
                        return Err(errors::MissingImageIdError { id: img_id });
                    },
                    counts: counts.remove(0),
                });
            };

            anns.insert(annotation.id, annotation);
            img_to_anns.entry(img_id).or_insert_with(Vec::new);
            img_to_anns
                .get_mut(&img_id)
                .expect("Image id not in the hashmap, eventhough it should have been initialized on the previous line.")
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
    pub fn get_ann<'a>(
        &'a self,
        ann_id: u32,
    ) -> Result<&'a Annotation, errors::MissingAnnotationIdError> {
        let ann = match self.anns.get(&ann_id) {
            None => return Err(errors::MissingAnnotationIdError { id: ann_id }),
            Some(ann) => ann,
        };

        Ok(ann)
    }

    /// Return a result containing the category struct corresponding to the given id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is no entry corresponding to `cat_id`.
    pub fn get_cat<'a>(
        &'a self,
        cat_id: u32,
    ) -> Result<&'a Category, errors::MissingCategoryIdError> {
        let cat = match self.cats.get(&cat_id) {
            None => return Err(errors::MissingCategoryIdError { id: cat_id }),
            Some(cat) => cat,
        };

        Ok(cat)
    }

    /// Return a result containing the image struct corresponding to the given image id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is no entry corresponding to `img_id`.
    pub fn get_img<'a>(&'a self, img_id: u32) -> Result<&'a Image, errors::MissingImageIdError> {
        let img = match self.imgs.get(&img_id) {
            None => return Err(errors::MissingImageIdError { id: img_id }),
            Some(img) => img,
        };

        Ok(img)
    }

    /// Return a result containing the annotations for the given image id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there is no entry corresponding to `img_id`.
    pub fn get_img_anns<'a>(
        &'a self,
        img_id: u32,
    ) -> Result<Vec<&'a Annotation>, errors::MissingImageIdError> {
        let mut anns: Vec<&Annotation> = Vec::new();
        match self.img_to_anns.get(&img_id) {
            None => return Err(errors::MissingImageIdError { id: img_id }),
            Some(ann_ids) => {
                for ann_id in ann_ids {
                    anns.push(self.get_ann(*ann_id).expect("The img_to_anns should not contain annotation ids that are not present in the anns hashmap."));
                }
            }
        }
        Ok(anns)
    }
}

/// # Panics
///
/// Will panic if the json file does not exists, cannot be opened or if an error happens when creating a Hashmap version of it.
#[must_use]
pub fn load_json(annotations_path: &String) -> HashmapDataset {
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
