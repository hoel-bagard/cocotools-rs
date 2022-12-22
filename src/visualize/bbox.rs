use crate::annotations::coco_types;
use image;
use imageproc::{drawing::draw_hollow_rect_mut, rect::Rect};

pub fn draw_bbox(img: &mut image::RgbImage, bbox: &coco_types::Bbox, color: &image::Rgb<u8>) {
    let rect =
        Rect::at(bbox.left as i32, bbox.top as i32).of_size(bbox.width as u32, bbox.height as u32);

    draw_hollow_rect_mut(img, rect, *color);
}
