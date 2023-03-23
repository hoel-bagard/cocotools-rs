use std::iter::zip;

use image;
use imageproc::{drawing::draw_hollow_rect_mut, rect::Rect};
use rand::Rng;

use crate::annotations::coco;
use crate::annotations::coco::Annotation;
use crate::converters::masks;
use crate::errors::MaskError;

/// Draw the bounding box on the image.
///
/// ## Args
/// - `img`: The image to draw on.
/// - `bbox`: The bounding box to draw.
/// - `color`: The color to use for drawing the bounding box.
///
/// # Example
///
/// ```rust
/// # use cocotools::annotations::coco::Bbox;
/// # use cocotools::visualize::draw::draw_bbox;
/// # use image::RgbImage;
/// let mut img = RgbImage::new(60, 60);
/// let bbox = Bbox{left: 40.0, top: 40.0, width: 10.0, height: 10.0};
/// let color = image::Rgb([255, 0, 0]);
/// draw_bbox(&mut img, &bbox, color);
/// ```
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn draw_bbox(img: &mut image::RgbImage, bbox: &coco::Bbox, color: image::Rgb<u8>) {
    let rect =
        Rect::at(bbox.left as i32, bbox.top as i32).of_size(bbox.width as u32, bbox.height as u32);

    draw_hollow_rect_mut(img, rect, color);
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn draw_mask(img: &mut image::RgbImage, mask: &masks::Mask, color: image::Rgb<u8>) {
    let mask_alpha: f64 = 0.4;
    let img_alpha = 1.0 - mask_alpha;
    for (image::Rgb([r, g, b]), mask_value) in zip(img.pixels_mut(), mask.iter()) {
        if *mask_value != 0 {
            *r = img_alpha.mul_add(f64::from(*r), mask_alpha * f64::from(color[0])) as u8;
            *g = img_alpha.mul_add(f64::from(*g), mask_alpha * f64::from(color[1])) as u8;
            *b = img_alpha.mul_add(f64::from(*b), mask_alpha * f64::from(color[2])) as u8;
        }
    }
}

/// Draw the segmentation masks, and optionnaly the bounding boxes of the annotations on the image.
///
/// # Errors
///
/// Will return `Err` if the segmentation annotations could not be decompressed.
pub fn draw_anns(
    img: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    anns: &Vec<&Annotation>,
    draw_bbox: bool,
) -> Result<(), MaskError> {
    let mut rng = rand::thread_rng();
    for ann in anns {
        let color = image::Rgb([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()]);
        if draw_bbox {
            self::draw_bbox(img, &ann.bbox, color);
        }
        let mask = masks::Mask::try_from(&ann.segmentation)?;
        draw_mask(img, &mask, color);
    }

    Ok(())
}

/// Note: implement this as a trait when adding support for grayscale.
pub fn draw_rgb_to_buffer(img: &image::RgbImage, dst: &mut [u32]) {
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
