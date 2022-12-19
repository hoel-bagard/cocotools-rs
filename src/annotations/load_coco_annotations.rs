use crate::annotations;
use crate::annotations::coco_types::Dataset;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

pub fn load_json(annotations_path: &String) {
    println!("{:?}", annotations_path);
    let annotations_file_content = fs::read_to_string(annotations_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            panic!("Could not find the annotations file: {:?}", error);
        } else {
            panic!("Problem opening the annotations file: {:?}", error);
        }
    });

    let dataset: Dataset =
        serde_json::from_str(&annotations_file_content).expect("Error decoding the json file");
    println!("{:?}", dataset.annotations[0]);
}
