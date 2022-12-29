use crate::annotations::coco_types;
use image::{GrayImage, ImageBuffer, Luma};

/// A boolean mask indicating for each pixel whether it belongs to the object or not.
pub type Mask = image::GrayImage;

impl From<&coco_types::RLE> for Mask {
    /// Converts a RLE to its uncompressed mask.
    fn from(rle: &coco_types::RLE) -> Self {
        let mut current_value = 0u8;
        let mut current_position = 0u32;
        let mut flat_mask = vec![0u8; usize::try_from(rle.size[1] * rle.size[0]).unwrap()];
        for nb_pixels in &rle.counts {
            for i in current_position..current_position + nb_pixels {
                flat_mask[i as usize] = current_value
            }
            current_value = if current_value == 0 { 1 } else { 0 };
            current_position += nb_pixels;
        }
        let mut mask = ImageBuffer::from_raw(rle.size[1], rle.size[0], flat_mask).unwrap();
        for pixel in mask.pixels_mut() {
            *pixel = image::Luma([255u8]);
        }

        mask.save("outputs/mask.jpg").unwrap_or_else(|error| {
            panic!("Could not save the image: {:?}", error);
        });

        println!("aaaaaaaaaaaaaaa");
        println!("aaaaaaaaaaaaaaa");

        mask
    }
}

impl From<&coco_types::Segmentation> for Mask {
    fn from(coco_segmentation: &coco_types::Segmentation) -> Self {
        let mask = match coco_segmentation {
            coco_types::Segmentation::RLE(rle) => Mask::from(rle),
            coco_types::Segmentation::EncodedRLE(encoded_rle) => {
                Mask::from(&coco_types::RLE::from(encoded_rle))
            }
            coco_types::Segmentation::Polygon(_) => GrayImage::new(10, 10),
        };
        mask
    }
}

// pub fn draw_mask(
//     img: &mut image::RgbImage,
//     mask: &coco_types::Segmentation,
//     color: &image::Rgb<u8>,
// ) {
//     let mask = match mask {
//         coco_types::Segmentation::EncodedRLE => Mask::from(mask);
//         coco_types::Segmentation::Polygon() => {}
//         coco_types::Segmentation::RLE => {}
//     };
// }
