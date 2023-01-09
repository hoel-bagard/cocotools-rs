pub mod bbox;
pub mod segmentation;

use crate::annotations::load_coco::HashmapDataset;
use crate::errors;
use image::io::Reader as ImageReader;
use rand::Rng;
use std::path::Path;

extern crate image;
extern crate minifb;

use image::*;
use minifb::{Key, Window, WindowOptions};
use std::cmp;

pub trait DrawExt {
    fn draw_to_buffer(&self, dst: &mut [u32], dst_width: usize, offset: (i32, i32));
}

impl DrawExt for RgbImage {
    fn draw_to_buffer(&self, dst: &mut [u32], dst_width: usize, offset: (i32, i32)) {
        let dst_size = (dst_width as i32, (dst.len() / dst_width) as i32);

        let (width, height) = self.dimensions();

        // Make sure only the pixels get rendered that are inside the dst
        let min_x = cmp::max(-offset.0, 0);
        let min_y = cmp::max(-offset.1, 0);

        let max_x = cmp::min(dst_size.0 - offset.0, width as i32);
        let max_y = cmp::min(dst_size.1 - offset.1, height as i32);

        for y in min_y..max_y {
            for x in min_x..max_x {
                let pixel = self.get_pixel(x as u32, y as u32);

                // Convert pixel to Color
                let raw = 0xFF000000
                    | ((pixel[0] as u32) << 16)
                    | ((pixel[1] as u32) << 8)
                    | (pixel[2] as u32);

                // Apply the offsets
                let dst_x = (x + offset.0) as usize;
                let dst_y = (y + offset.1) as usize;

                // Calculate the index
                let index = dst_x + dst_y * dst_size.0 as usize;
                dst[index] = raw;
            }
        }
    }
}

/// # Panics
///
/// Will panic if it cannot read the image file corresponding to the `img_id`.
///
/// # Errors
///
/// Will return `Err` if `img_id` is not present in the dataset.
pub fn visualize_img(
    dataset: &HashmapDataset,
    image_folder: &String,
    img_id: u32,
) -> Result<(), errors::MissingIdError> {
    let img_name = &dataset.get_img(img_id)?.file_name;
    let sample_path = Path::new(image_folder).join(img_name);

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
    for ann in dataset.get_img_anns(img_id)? {
        let color = image::Rgb([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()]);
        bbox::draw_bbox(&mut img, &ann.bbox, color);
        let mask = segmentation::Mask::from(&ann.segmentation);
        segmentation::draw_mask(&mut img, &mask, color);
    }

    let img_width = img.width() as usize;
    let img_height = img.height() as usize;
    let mut buffer: Vec<u32> = vec![0x00FFFFFF; img_width * img_height];
    img.draw_to_buffer(&mut buffer, img_width, (0, 0));

    let mut window = Window::new(
        format!("{img_name} - Press Q or ESC to exit",).as_str(),
        img_width,
        img_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Could not open window, got the following error: {}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        window
            .update_with_buffer(&buffer, img_width, img_height)
            .unwrap();
    }

    Ok(())
}
