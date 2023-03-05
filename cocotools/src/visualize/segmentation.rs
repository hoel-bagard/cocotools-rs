use std::iter::zip;

use image::{Luma, Rgb};
use imageproc::drawing;

use crate::annotations::coco;

/// A boolean mask indicating for each pixel whether it belongs to the object or not.
pub type Mask = image::GrayImage;

impl From<&coco::Segmentation> for Mask {
    fn from(coco_segmentation: &coco::Segmentation) -> Self {
        match coco_segmentation {
            coco::Segmentation::Rle(rle) => Self::from(rle),
            coco::Segmentation::EncodedRle(encoded_rle) => {
                Self::from(&coco::Rle::from(encoded_rle))
            }
            coco::Segmentation::PolygonRS(poly) => Self::from(poly),
            coco::Segmentation::Polygon(_) => {
                unimplemented!("Use the 'mask_from_poly' function.")
            }
        }
    }
}

impl From<&coco::Rle> for Mask {
    /// Converts a RLE to its uncompressed mask.
    fn from(rle: &coco::Rle) -> Self {
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

#[allow(clippy::cast_possible_truncation)]
impl From<&coco::PolygonRS> for Mask {
    /// Converts a polygon representation of a mask to an RLE one.
    fn from(poly: &coco::PolygonRS) -> Self {
        let mut points_poly: Vec<imageproc::point::Point<i32>> = Vec::new();
        for i in (0..poly.counts.len()).step_by(2) {
            points_poly.push(imageproc::point::Point::new(
                poly.counts[i] as i32,
                poly.counts[i + 1] as i32,
            ));
        }
        if let Some(last_point) = points_poly.last() {
            if points_poly[0].x == last_point.x && points_poly[0].y == last_point.y {
                points_poly.pop();
            }
        }

        let mut mask = Self::new(poly.size[1], poly.size[0]);
        drawing::draw_polygon_mut(&mut mask, &points_poly, image::Luma([1u8]));

        mask
    }
}

#[allow(clippy::cast_possible_truncation)]
pub fn mask_from_poly(poly: &coco::Polygon, width: u32, height: u32) -> Mask {
    let mut points_poly: Vec<imageproc::point::Point<i32>> = Vec::new();
    for i in (0..poly[0].len()).step_by(2) {
        points_poly.push(imageproc::point::Point::new(
            poly[0][i] as i32,
            poly[0][i + 1] as i32,
        ));
    }
    let mut mask = image::GrayImage::new(width, height);
    drawing::draw_polygon_mut(&mut mask, &points_poly, image::Luma([1u8]));

    mask
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
