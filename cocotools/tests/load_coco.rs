use std::path::PathBuf;

use cocotools::COCO;

#[test]
fn load_from_file() {
    let annotations_file_path = PathBuf::from("../data_samples/coco_25k/annotations.json");
    let dataset = COCO::try_from(&annotations_file_path).unwrap();

    assert_eq!(
        dataset.get_img(17627).unwrap().file_name,
        "000000017627.jpg"
    );
    assert_eq!(dataset.get_ann(128189).unwrap().area, 71436.89385);
    assert_eq!(dataset.get_cat(86).unwrap().name, "vase");
}
