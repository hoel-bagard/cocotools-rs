use crate::annotations::coco_types::BBox;
use image::Rgb;
use image::RgbImage;
use imageproc::{drawing::draw_hollow_rect_mut, rect::Rect};

pub fn draw_bbox(img: &mut image::RgbImage) {
    let bbox = BBox {
        left: 100.0,
        top: 100.0,
        width: 100.0,
        height: 100.0,
    };
    let rect =
        Rect::at(bbox.left as i32, bbox.top as i32).of_size(bbox.width as u32, bbox.height as u32);

    let white = Rgb([0, 0, 0]);

    draw_hollow_rect_mut(img, rect, white);
}

// https://docs.rs/image/latest/image/
// https://crates.io/crates/image
