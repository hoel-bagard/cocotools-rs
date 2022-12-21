use crate::annotations::coco_types::{Annotation, Bbox, Category, Dataset, Image, Segmentation};
use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;

#[derive(Debug)]
pub struct HashmapDataset {
    pub anns: HashMap<u32, Annotation>,
    pub imgs: HashMap<u32, Image>,
    pub cats: HashMap<u32, Category>,
    /// Hashmap that links an image id to the image's annotations
    img_to_anns: HashMap<u32, Vec<u32>>,
    // TODO: Is it possible to have img_to_anns map directly to a Vec of Annotations.
    // pub img_to_anns: HashMap<u32, Vec<&'a Annotation>>,
}

impl<'a> HashmapDataset {
    pub fn new(dataset: Dataset) -> Self {
        let mut anns: HashMap<u32, Annotation> = HashMap::new();
        let mut imgs: HashMap<u32, Image> = HashMap::new();
        let mut cats: HashMap<u32, Category> = HashMap::new();
        let mut img_to_anns: HashMap<u32, Vec<u32>> = HashMap::new();

        for annotation in dataset.annotations {
            let ann_id = annotation.id.clone();
            let img_id = annotation.image_id.clone();
            anns.insert(annotation.id, annotation);
            if !img_to_anns.contains_key(&img_id) {
                img_to_anns.insert(img_id, Vec::new());
            }
            img_to_anns.get_mut(&img_id).unwrap().push(ann_id);
        }

        for image in dataset.images {
            imgs.insert(image.id, image);
        }

        for category in dataset.categories {
            cats.insert(category.id, category);
        }

        HashmapDataset {
            anns,
            imgs,
            cats,
            img_to_anns,
        }
    }

    pub fn get_ann(&'a self, ann_id: &u32) -> &'a Annotation {
        self.anns.get(ann_id).unwrap_or_else(|| {
            panic!("The dataset does not contain an annotation with id {ann_id}");
        })
    }

    pub fn get_img(&'a self, img_id: &u32) -> &'a Image {
        self.imgs.get(img_id).unwrap_or_else(|| {
            panic!("The dataset does not contain an annotation with id {img_id}");
        })
    }

    pub fn get_cat(&'a self, cat_id: &u32) -> &'a Category {
        self.cats.get(cat_id).unwrap_or_else(|| {
            panic!("The dataset does not contain an annotation with id {cat_id}");
        })
    }

    pub fn get_img_anns(&'a self, img_id: &u32) -> Vec<&'a Annotation> {
        let mut anns: Vec<&Annotation> = Vec::new();
        for ann_id in self.img_to_anns.get(img_id).unwrap() {
            anns.push(self.get_ann(ann_id))
        }
        anns
    }
}

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
