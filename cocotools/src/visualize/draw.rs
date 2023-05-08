use std::iter::zip;

use image;
use imageproc::{drawing::draw_hollow_rect_mut, rect::Rect};
use rand::Rng;

use crate::coco::object_detection;
use crate::errors::MaskError;
use crate::mask;

/// Draw the bounding box on the image.
///
/// ## Args
/// - `img`: The image to draw on.
/// - `bbox`: The bounding box to draw.
/// - `color`: The color to use for drawing the bounding box.
///
/// ## Example
///
/// ```rust
/// # use image::RgbImage;
/// # use cocotools::coco::object_detection::Bbox;
/// use cocotools::visualize::draw;
/// let mut img = RgbImage::new(60, 60);
/// let bbox = Bbox{left: 40.0, top: 40.0, width: 10.0, height: 10.0};
/// let color = image::Rgb([255, 0, 0]);
/// draw::bbox(&mut img, &bbox, color);
/// ```
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn bbox(img: &mut image::RgbImage, bbox: &object_detection::Bbox, color: image::Rgb<u8>) {
    if bbox.width > 0.0 && bbox.height > 0.0 {
        let rect = Rect::at(bbox.left as i32, bbox.top as i32)
            .of_size(bbox.width as u32, bbox.height as u32);

        draw_hollow_rect_mut(img, rect, color);
    }
}

/// Draw the max on the image.
///
/// ## Args
/// - `img`: The image to draw on.
/// - `mask`: The mask to draw.
/// - `color`: The color to use for drawing the mask.
///
/// ## Example
///
/// ```rust
/// # use image::RgbImage;
/// # use ndarray::array;
/// # use cocotools::coco::object_detection::Bbox;
/// use cocotools::visualize::draw;
/// let mask = &array![[0, 0, 0, 0, 0, 0, 0],
///                    [0, 0, 1, 1, 1, 0, 0],
///                    [0, 0, 1, 1, 1, 0, 0],
///                    [0, 0, 1, 1, 1, 0, 0],
///                    [0, 0, 1, 1, 1, 0, 0],
///                    [0, 0, 1, 1, 1, 0, 0],
///                    [0, 0, 0, 0, 0, 0, 0]];
/// let mut img = RgbImage::new(7, 7);
/// let color = image::Rgb([255, 0, 0]);
/// draw::mask(&mut img, &mask, color);
/// ```
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn mask(img: &mut image::RgbImage, mask: &mask::Mask, color: image::Rgb<u8>) {
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
/// ## Args
/// - `img`: The image to draw on.
/// - `anns`: The annotations to draw. They are assumed to correspong to the image, or to an image of the same size as `img`.
/// - `draw_bbox`: If true, then also the bounding boxes.
///
/// # Example
///
/// ```rust
/// # use cocotools::coco::object_detection;
/// # use image::RgbImage;
/// use cocotools::visualize::draw;
/// let mut img = RgbImage::new(40, 40);
/// let anns = vec![
///     object_detection::Annotation {
///         id: 1,
///         image_id: 1,
///         category_id: 1,
///         segmentation: object_detection::Segmentation::CocoRle(object_detection::CocoRle {
///             size: vec![40, 40],
///             counts: "e75S10000000ST1".to_string(),
///         }),
///         // # the bounding box here does not correspond to the segmentation.
///         area: 1.0,
///         bbox: object_detection::Bbox {
///             left: 10.0,
///             top: 10.0,
///             width: 20.0,
///             height: 20.0,
///         },
///         iscrowd: 0,
///     },
///     object_detection::Annotation {
///         id: 2,
///         image_id: 1,
///         category_id: 2,
///         segmentation: object_detection::Segmentation::PolygonsRS(object_detection::PolygonsRS {
///             size: vec![40, 40],
///             counts: vec![vec![4.0, 4.0, 24.0, 4.0, 24.0, 24.0, 4.0, 24.0]],
///         }),
///         area: 400.0,
///         bbox: object_detection::Bbox {
///             left: 4.0,
///             top: 4.0,
///             width: 24.0,
///             height: 24.0,
///         },
///         iscrowd: 0,
///     },
/// ];
/// draw::anns(&mut img, &anns.iter().collect(), true);
/// ```
///
/// ## Errors
///
/// Will return `Err` if the segmentation annotations could not be decompressed.
pub fn anns(
    img: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    anns: &Vec<&object_detection::Annotation>,
    draw_bbox: bool,
) -> Result<(), MaskError> {
    let mut rng = rand::thread_rng();
    for ann in anns {
        let color = image::Rgb([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()]);
        if draw_bbox {
            self::bbox(img, &ann.bbox, color);
        }
        let mask = mask::Mask::try_from(&ann.segmentation)?;
        self::mask(img, &mask, color);
    }

    Ok(())
}

pub(super) trait ToBuffer {
    fn to_buffer(&self) -> Vec<u32>;
}

/// Write `img` into a a buffer (vector) and returns it.
///
/// ## Example
///
/// ```ignore
/// # use cocotools::visualize::draw::rgb_to_buffer;
/// # use image::RgbImage;
/// let img = RgbImage::new(40, 40);
/// let buffer = img.to_buffer()
/// ```
impl ToBuffer for image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    fn to_buffer(&self) -> Vec<u32> {
        let mut buffer: Vec<u32> = vec![0x00FF_FFFF; (self.width() * self.height()) as usize];
        for x in 0..self.width() {
            for y in 0..self.height() {
                let pixel = self.get_pixel(x, y);

                // Convert pixel to 0RGB
                let raw = 0xFF00_0000
                    | (u32::from(pixel[0]) << 16)
                    | (u32::from(pixel[1]) << 8)
                    | u32::from(pixel[2]);

                // Calculate the index in the 1D dist buffer.
                let index = x + y * self.width();
                buffer[index as usize] = raw;
            }
        }
        buffer
    }
}
