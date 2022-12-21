pub mod bbox;
use crate::annotations::load_coco_annotations::load_json;
use crate::args::VisualizeSampleArgs;
use image::io::Reader as ImageReader;
use std::path::Path;

pub fn visualize_sample(sample_args: VisualizeSampleArgs) {
    let dataset = load_json(&sample_args.annotation_file);

    let sample_path = Path::new(&sample_args.image_folder)
        .join(&dataset.get_img(&sample_args.sample_id).file_name);

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

    // dataset.get_ann(sample_args.sample_id).bbox
    bbox::draw_bbox(&mut img);

    img.save("out.jpg").unwrap_or_else(|error| {
        panic!("Could not save the image: {:?}", error);
    });
}
