pub mod bbox;
pub mod segmentation;

use crate::annotations::coco::{Annotation, HashmapDataset};
use crate::converters::masks;

use image::io::Reader as ImageReader;
use rand::Rng;

use std::path::{Path, PathBuf};

extern crate image;
extern crate minifb;

use minifb::{Key, Window, WindowOptions};

/// Note: implement this as a trait when adding support for grayscale.
fn draw_rgb_to_buffer(img: &image::RgbImage, dst: &mut [u32]) {
    for x in 0..img.width() {
        for y in 0..img.height() {
            let pixel = img.get_pixel(x, y);

            // Convert pixel to 0RGB
            let raw = 0xFF00_0000
                | (u32::from(pixel[0]) << 16)
                | (u32::from(pixel[1]) << 8)
                | u32::from(pixel[2]);

            // Calculate the index in the 1D dist buffer.
            let index = x + y * img.width();
            dst[index as usize] = raw;
        }
    }
}

/// Visualize the labels for the given image id.
///
/// # Errors
///
/// Will return `Err` if `img_id` is not present in the dataset.
pub fn visualize_img(
    dataset: &HashmapDataset,
    image_folder: &Path,
    img_id: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let anns = dataset.get_img_anns(img_id)?;
    let img_name = &dataset.get_img(img_id)?.file_name;
    let img_path = image_folder.join(img_name);

    show_anns(&img_path, anns, true)?;

    Ok(())
}

/// Visualize the given image and annotations.
///
/// # Arguments
///
/// * `img_path` - The path to the image corresponding to the annotations.
/// * `anns` - The annotations to draw on the image.
/// * `draw_bbox` - If true, draw the bounding boxes.
///
/// # Panics
///
/// Will panic if it cannot read the image file corresponding to the `img_id`.
///
/// # Errors
///
/// Will return `Err` if the COCO segmentation mask decompression fails.
pub fn show_anns(
    img_path: &PathBuf,
    anns: Vec<&Annotation>,
    draw_bbox: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = ImageReader::open(img_path)
        .unwrap_or_else(|error| {
            panic!(
                "Could not open the image {}: {:?}",
                img_path.display(),
                error
            );
        })
        .decode()
        .unwrap_or_else(|error| {
            panic!(
                "Could not decode the image {}: {:?}",
                img_path.display(),
                error
            );
        })
        .into_rgb8();

    draw_anns(&mut img, anns, draw_bbox)?;

    let img_width = img.width() as usize;
    let img_height = img.height() as usize;
    let mut buffer: Vec<u32> = vec![0x00FF_FFFF; img_width * img_height];
    draw_rgb_to_buffer(&img, &mut buffer);
    let mut window = Window::new(
        format!(
            "{} - Press Q or ESC to exit",
            img_path
                .file_name()
                .map_or("Image", |file_name| file_name.to_str().unwrap_or("Image"))
        )
        .as_str(),
        img_width,
        img_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Could not open window, got the following error: {e}");
    });

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window
            .update_with_buffer(&buffer, img_width, img_height)
            .unwrap_or_else(|e| {
                panic!("Could not update buffer, got the following error: {e}");
            });
    }
    Ok(())
}

/// Draw the segmentation masks, and optionnaly the bounding boxes of the annotations on the image.
pub fn draw_anns(
    mut img: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    anns: Vec<&Annotation>,
    draw_bbox: bool,
) -> Result<(), masks::MaskError> {
    let mut rng = rand::thread_rng();
    for ann in anns {
        let color = image::Rgb([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()]);
        if draw_bbox {
            bbox::draw_bbox(&mut img, &ann.bbox, color);
        }
        let mask = masks::Mask::try_from(&ann.segmentation)?;
        segmentation::draw_mask(&mut img, &mask, color);
    }

    Ok(())
}
