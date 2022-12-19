use crate::annotations::coco_types::BBox;

pub fn draw_bbox() {
    let bbox = BBox {
        left: 1.0,
        top: 1.0,
        width: 1.0,
        height: 1.0,
    };
}

// https://docs.rs/image/latest/image/
// https://crates.io/crates/image
