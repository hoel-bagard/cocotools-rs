use std::path::PathBuf;

use cocotools::COCO;

#[test]
#[allow(clippy::unwrap_used, clippy::float_cmp)]
fn load_from_file() {
    let annotations_file_path = PathBuf::from("../data_samples/coco_25k/annotations.json");
    let image_folder_path = PathBuf::from("../data_samples/coco_25k/images");
    let dataset = COCO::new(&annotations_file_path, &image_folder_path).unwrap();

    assert_eq!(
        dataset.get_img(17627).unwrap().file_name,
        "000000017627.jpg"
    );
    assert_eq!(dataset.get_ann(128_189).unwrap().area, 71436.89385);
    assert_eq!(dataset.get_cat(86).unwrap().name, "vase");
}
