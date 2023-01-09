use crate::annotations::coco_types;
use image::{Luma, Rgb};
use std::iter::zip;

/// A boolean mask indicating for each pixel whether it belongs to the object or not.
pub type Mask = image::GrayImage;

impl From<&coco_types::Rle> for Mask {
    /// Converts a RLE to its uncompressed mask.
    fn from(rle: &coco_types::Rle) -> Self {
        let mut mask = Self::new(rle.size[1], rle.size[0]);
        let mut current_value = 0u8;
        let mut x = 0u32;
        let mut y = 0u32;
        for nb_pixels in &rle.counts {
            for _ in 0..*nb_pixels {
                mask.put_pixel(x, y, Luma([current_value * 255]));
                y += 1;
                if y == rle.size[0] {
                    y = 0;
                    x += 1;
                }
            }
            current_value = u8::from(current_value == 0);
        }
        mask
    }
}

impl From<&coco_types::Segmentation> for Mask {
    fn from(coco_segmentation: &coco_types::Segmentation) -> Self {
        match coco_segmentation {
            coco_types::Segmentation::Rle(rle) => Self::from(rle),
            coco_types::Segmentation::EncodedRle(encoded_rle) => {
                Self::from(&coco_types::Rle::from(encoded_rle))
            }
            coco_types::Segmentation::Polygon(_) => Self::new(10, 10),
        }
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn draw_mask(img: &mut image::RgbImage, mask: &Mask, color: image::Rgb<u8>) {
    let mask_alpha: f64 = 0.4;
    let img_alpha = 1.0 - mask_alpha;
    for (Rgb([r, g, b]), Luma([mask])) in zip(img.pixels_mut(), mask.pixels()) {
        if *mask != 0 {
            *r = img_alpha.mul_add(f64::from(*r), mask_alpha * f64::from(color[0])) as u8;
            *g = img_alpha.mul_add(f64::from(*g), mask_alpha * f64::from(color[1])) as u8;
            *b = img_alpha.mul_add(f64::from(*b), mask_alpha * f64::from(color[2])) as u8;
        }
    }
}
