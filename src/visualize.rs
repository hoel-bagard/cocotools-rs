pub mod bbox;
use crate::annotations::load_coco_annotations::load_json;
use crate::args::VisualizeSampleArgs;
use image;
use image::io::Reader as ImageReader;
use rand::Rng;
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

    let mut rng = rand::thread_rng();
    for ann in dataset.get_img_anns(&sample_args.sample_id) {
        let color = image::Rgb([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()]);
        bbox::draw_bbox(&mut img, &ann.bbox, &color);
    }

    img.save("outputs/out.jpg").unwrap_or_else(|error| {
        panic!("Could not save the image: {:?}", error);
    });
}
