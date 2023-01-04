use crate::annotations::coco_types::{Annotation, Category, Dataset, Image};
use crate::errors;
use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;

#[derive(Debug)]
pub struct HashmapDataset {
    anns: HashMap<u32, Annotation>,
    cats: HashMap<u32, Category>,
    imgs: HashMap<u32, Image>,
    /// Hashmap that links an image id to the image's annotations
    img_to_anns: HashMap<u32, Vec<u32>>,
    // TODO: Is it possible to have img_to_anns map directly to a Vec of Annotations.
    // pub img_to_anns: HashMap<u32, Vec<&'a Annotation>>,
}

impl<'a> HashmapDataset {
    pub fn new(dataset: Dataset) -> Self {
        let mut anns: HashMap<u32, Annotation> = HashMap::new();
        let mut cats: HashMap<u32, Category> = HashMap::new();
        let mut imgs: HashMap<u32, Image> = HashMap::new();
        let mut img_to_anns: HashMap<u32, Vec<u32>> = HashMap::new();

        for annotation in dataset.annotations {
            let ann_id = annotation.id;
            let img_id = annotation.image_id;
            anns.insert(annotation.id, annotation);
            img_to_anns.entry(img_id).or_insert_with(Vec::new);
            img_to_anns
                .get_mut(&img_id)
                .expect("Image id not in the hashmap, eventhough it should have been initialized on the previous line.")
                .push(ann_id);
        }

        for category in dataset.categories {
            cats.insert(category.id, category);
        }

        for image in dataset.images {
            imgs.insert(image.id, image);
        }

        Self {
            anns,
            cats,
            imgs,
            img_to_anns,
        }
    }

    /// Returns the annotation with the given ID, or an error if the dataset does not contain such an annotation.
    pub fn get_ann(
        &'a self,
        ann_id: u32,
    ) -> Result<&'a Annotation, errors::MissingAnnotationIdError> {
        let ann = match self.anns.get(&ann_id) {
            None => return Err(errors::MissingAnnotationIdError { id: ann_id }),
            Some(ann) => ann,
        };

        Ok(ann)
    }

    pub fn get_cat(&'a self, cat_id: u32) -> Result<&'a Category, errors::MissingCategoryIdError> {
        let cat = match self.cats.get(&cat_id) {
            None => return Err(errors::MissingCategoryIdError { id: cat_id }),
            Some(cat) => cat,
        };

        Ok(cat)
    }

    pub fn get_img(&'a self, img_id: u32) -> Result<&'a Image, errors::MissingImageIdError> {
        let img = match self.imgs.get(&img_id) {
            None => return Err(errors::MissingImageIdError { id: img_id }),
            Some(img) => img,
        };

        Ok(img)
    }

    /// Return the annotations for the given image id, or None if there is no annotation corresponding to the given image id.
    pub fn get_img_anns(
        &'a self,
        img_id: u32,
    ) -> Result<Vec<&'a Annotation>, errors::MissingImageIdError> {
        let mut anns: Vec<&Annotation> = Vec::new();
        match self.img_to_anns.get(&img_id) {
            None => return Err(errors::MissingImageIdError { id: img_id }),
            Some(ann_ids) => {
                for ann_id in ann_ids {
                    anns.push(self.get_ann(*ann_id).expect("The img_to_anns should not contain annotation ids that are not present in the anns hashmap."))
                }
            }
        }
        Ok(anns)
    }
}

/// # Panics
///
/// Will panic if the json file does not exists or cannot be opened.
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

    HashmapDataset::new(dataset)
}
