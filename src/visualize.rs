pub mod bbox;
pub mod segmentation;

use crate::annotations::load_coco_annotations::HashmapDataset;
use image::io::Reader as ImageReader;
use rand::Rng;
use std::path::Path;

pub fn visualize_sample(dataset: &HashmapDataset, image_folder: &String, sample_id: u32) {
    let sample_path = Path::new(image_folder).join(&dataset.get_img(&sample_id).file_name);

    let mut img = ImageReader::open(&sample_path)
        .unwrap_or_else(|error| {
            panic!(
                "Could not open the image {}: {:?}",
                sample_path.display(),
                error
            );
        })
        .decode()
        .unwrap_or_else(|error| {
            panic!(
                "Could not decode the image {}: {:?}",
                sample_path.display(),
                error
            );
        })
        .into_rgb8();

    let mut rng = rand::thread_rng();
    for ann in dataset.get_img_anns(&sample_id) {
        let color = image::Rgb([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()]);
        bbox::draw_bbox(&mut img, &ann.bbox, color);
        let mask = segmentation::Mask::from(&ann.segmentation);
        segmentation::draw_mask(&mut img, &mask, color);
    }

    // Use show_image or viuer here.
    img.save("outputs/out.jpg").unwrap_or_else(|error| {
        panic!("Could not save the image: {:?}", error);
    });
}
