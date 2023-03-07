use std::iter::zip;

use image::Rgb;

use crate::converters::masks::Mask;

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn draw_mask(img: &mut image::RgbImage, mask: &Mask, color: image::Rgb<u8>) {
    let mask_alpha: f64 = 0.4;
    let img_alpha = 1.0 - mask_alpha;
    for (Rgb([r, g, b]), mask_value) in zip(img.pixels_mut(), mask.iter()) {
        if *mask_value != 0 {
            *r = img_alpha.mul_add(f64::from(*r), mask_alpha * f64::from(color[0])) as u8;
            *g = img_alpha.mul_add(f64::from(*g), mask_alpha * f64::from(color[1])) as u8;
            *b = img_alpha.mul_add(f64::from(*b), mask_alpha * f64::from(color[2])) as u8;
        }
    }
}
