use crate::annotations::coco_types::{Annotation, Bbox, Category, Dataset, Image, Segmentation};
use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;

// def createIndex(self):
//     # create index
//     print('creating index...')
//     anns, cats, imgs = {}, {}, {}
//     imgToAnns,catToImgs = defaultdict(list),defaultdict(list)
//     if 'annotations' in self.dataset:
//         for ann in self.dataset['annotations']:
//             imgToAnns[ann['image_id']].append(ann)
//             anns[ann['id']] = ann

//     if 'images' in self.dataset:
//         for img in self.dataset['images']:
//             imgs[img['id']] = img

//     if 'categories' in self.dataset:
//         for cat in self.dataset['categories']:
//             cats[cat['id']] = cat

//     if 'annotations' in self.dataset and 'categories' in self.dataset:
//         for ann in self.dataset['annotations']:
//             catToImgs[ann['category_id']].append(ann['image_id'])

//     print('index created!')

//     # create class members
//     self.anns = anns
//     self.imgToAnns = imgToAnns
//     self.catToImgs = catToImgs
//     self.imgs = imgs
//     self.cats = cats

#[derive(Debug)]
pub struct HashmapDataset {
    pub anns: HashMap<u32, Annotation>,
    pub imgs: HashMap<u32, Image>,
    pub cats: HashMap<u32, Category>,
}

impl HashmapDataset {
    pub fn new(dataset: Dataset) -> Self {
        let mut anns: HashMap<u32, Annotation> = HashMap::new();
        let mut imgs: HashMap<u32, Image> = HashMap::new();
        let mut cats: HashMap<u32, Category> = HashMap::new();

        for annotation in dataset.annotations {
            anns.insert(annotation.id, annotation);
        }

        for image in dataset.images {
            imgs.insert(image.id, image);
        }

        for category in dataset.categories {
            cats.insert(category.id, category);
        }

        HashmapDataset { anns, imgs, cats }
    }
}

pub fn load_json(annotations_path: &String) -> Dataset {
    let annotations_file_content = fs::read_to_string(annotations_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            panic!("Could not find the annotations file: {:?}", error);
        } else {
            panic!("Problem opening the annotations file: {:?}", error);
        }
    });

    let dataset: Dataset =
        serde_json::from_str(&annotations_file_content).expect("Error decoding the json file");

    dataset
}

// pub fn get_img_ann(dataset: Dataset, id: u32) -> Annotation {
//     // Use a Hashmap to store the annotations.
//     // Use a trait / impl
// }
