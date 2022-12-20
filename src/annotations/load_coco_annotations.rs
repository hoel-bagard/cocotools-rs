use crate::annotations::coco_types::{Annotation, Dataset};
use std::fs;
use std::io::ErrorKind;

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
// }
