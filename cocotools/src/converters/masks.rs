use image;
use image::Luma;
use imageproc::drawing;
use thiserror::Error;

use crate::annotations::coco;
use crate::argparse::Segmentation;

/// # Errors
///
/// Will return `Err` if the conversion failed.
pub fn convert_coco_segmentation(
    dataset: &mut coco::HashmapDataset,
    target_segmentation: Segmentation,
) -> Result<(), MaskError> {
    let anns: Vec<coco::Annotation> = dataset.get_anns().into_iter().cloned().collect();
    for ann in anns {
        let converted_segmentation = match &ann.segmentation {
            coco::Segmentation::Rle(rle) => match target_segmentation {
                Segmentation::Rle => coco::Segmentation::Rle(rle.clone()),
                Segmentation::EncodedRle => {
                    coco::Segmentation::EncodedRle(coco::EncodedRle::try_from(rle)?)
                }
                Segmentation::Polygon => coco::Segmentation::Polygon(coco::Polygon::from(rle)),
            },
            coco::Segmentation::EncodedRle(_encoded_rle) => todo!(),
            coco::Segmentation::PolygonRS(poly) => match target_segmentation {
                Segmentation::Rle => coco::Segmentation::Rle(coco::Rle::from(poly)),
                Segmentation::EncodedRle => todo!(),
                Segmentation::Polygon => coco::Segmentation::Polygon(vec![poly.counts.clone()]),
            },
            coco::Segmentation::Polygon(_) => unimplemented!(),
        };
        dataset.add_ann(&coco::Annotation {
            segmentation: converted_segmentation,
            ..ann.clone()
        });
    }
    Ok(())
}

impl From<&coco::Rle> for coco::Polygon {
    fn from(_rle: &coco::Rle) -> Self {
        todo!()
    }
}

impl From<&coco::PolygonRS> for coco::Rle {
    // It might be more efficient to do it like this: https://github.com/cocodataset/cocoapi/blob/master/common/maskApi.c#L162
    // It would also avoid having slightly different results from the reference implementation.
    fn from(poly: &coco::PolygonRS) -> Self {
        coco::Rle::from(&Mask::from(poly))
    }
}

/// Decode encoded rle segmentation information into a rle.

/// See the (hard to read) implementation:
/// <https://github.com/cocodataset/cocoapi/blob/master/common/maskApi.c#L218>
/// <https://github.com/cocodataset/cocoapi/blob/8c9bcc3cf640524c4c20a9c40e89cb6a2f2fa0e9/PythonAPI/pycocotools/_mask.pyx#L145>

/// [LEB128 wikipedia article](https://en.wikipedia.org/wiki/LEB128#Decode_signed_integer)
/// It is similar to LEB128, but here shift is incremented by 5 instead of 7 because the implementation uses
/// 6 bits per byte instead of 8. (no idea why, I guess it's more efficient for the COCO dataset?)
#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
impl From<&coco::EncodedRle> for coco::Rle {
    /// Converts a compressed RLE to its uncompressed version.
    fn from(encoded_rle: &coco::EncodedRle) -> Self {
        assert!(
            encoded_rle.counts.is_ascii(),
            "Encoded RLE is not in valid ascii."
        );

        let bytes_rle = encoded_rle.counts.as_bytes();

        let mut current_count_idx: usize = 0;
        let mut current_byte_idx: usize = 0;
        let mut counts: Vec<u32> = vec![0; encoded_rle.counts.len()];
        while current_byte_idx < bytes_rle.len() {
            let mut continuous_pixels: i32 = 0;
            let mut shift = 0;
            let mut high_order_bit = 1;

            // When the high order bit of a byte becomes 0, we have decoded the integer and can move on to the next one.
            while high_order_bit != 0 {
                let byte = bytes_rle[current_byte_idx] - 48; // The encoding uses the ascii chars 48-111.

                // 0x1f is 31, i.e. 001111 --> Here we select the first four bits of the byte.
                continuous_pixels |= (i32::from(byte) & 31) << shift;
                // 0x20 is 32 as int, i.e. 2**5, i.e 010000 --> Here we select the fifth bit of the byte.
                high_order_bit = byte & 32;
                current_byte_idx += 1;
                shift += 5;
                // 0x10 is 16 as int, i.e. 1000
                if high_order_bit == 0 && (byte & 16 != 0) {
                    continuous_pixels |= !0 << shift;
                }
            }

            if current_count_idx > 2 {
                // My hypothesis as to what is happening here, is that most objects are going to be somewhat
                // 'vertically convex' (i.e. have only one continuous run per line).
                // In which case, the next 'row' of black/white pixels is going to be similar to the one preceding it.
                // Therefore, by having the continuous count of pixels be an offset of the one preceding it, we can have it be
                // a smaller int and therefore use less bits to encode it.
                continuous_pixels += counts[current_count_idx - 2] as i32;
            }
            counts[current_count_idx] = continuous_pixels as u32;
            current_count_idx += 1;
        }

        // TODO: Added the while loop to pass the tests, but it should not be there. Something is wrong somewhere else.
        while let Some(last) = counts.last() {
            if *last == 0 {
                counts.pop();
            } else {
                break;
            }
        }

        Self {
            size: encoded_rle.size.clone(),
            counts,
        }
    }
}

impl TryFrom<&coco::Rle> for coco::EncodedRle {
    type Error = MaskError;

    // Get compressed string representation of encoded mask.
    fn try_from(rle: &coco::Rle) -> Result<Self, Self::Error> {
        let mut high_order_bit: bool;
        let mut byte: u8;
        let mut encoded_counts: Vec<u8> = Vec::new();

        for i in 0..rle.counts.len() {
            let mut continuous_pixels = i64::from(rle.counts[i]);
            if i > 2 {
                continuous_pixels -= i64::from(rle.counts[i - 2]);
            }
            high_order_bit = true;
            while high_order_bit {
                byte = u8::try_from(continuous_pixels & 0x1f)
                    .map_err(|err| MaskError::IntConversion(err, continuous_pixels & 0x1f))?;
                continuous_pixels >>= 5;
                high_order_bit = if byte & 0x10 == 0 {
                    continuous_pixels != 0
                } else {
                    continuous_pixels != -1
                };
                if high_order_bit {
                    byte |= 0x20;
                };
                byte += 48;
                encoded_counts.push(byte);
            }
        }
        Ok(Self {
            size: rle.size.clone(),
            counts: std::str::from_utf8(&encoded_counts)
                .map_err(|err| MaskError::StrConversion(err, encoded_counts.clone()))?
                .to_string(),
        })
    }
}

/// Convert a mask into its RLE form.
///
/// ## Args:
/// - mask: A binary mask indicating for each pixel whether it belongs to the object or not.
///
/// ## Returns:
/// - The RLE corresponding to the mask.
impl From<&Mask> for coco::Rle {
    fn from(mask: &Mask) -> Self {
        let mut previous_value = 0;
        let mut count = 0;
        let mut counts = Vec::new();
        for pixel in mask.pixels() {
            if pixel[0] != previous_value {
                counts.push(count);
                previous_value = pixel[0];
                count = 0;
            }
            count += 1;
        }
        counts.push(count);

        Self {
            size: vec![mask.width(), mask.height()],
            counts,
        }
    }
}

/// A boolean mask indicating for each pixel whether it belongs to the object or not.
pub type Mask = image::GrayImage;
// pub type Mask = ImageBuffer<Luma<u8>, Vec<u8>>;

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
    /// Create a mask from a compressed polygon representation.
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

#[derive(Debug, Error)]
pub enum MaskError {
    #[error("Failed to convert RLE to its compressed version due to a type conversion error. Tried to convert '{1:?}' to u8 and failed.")]
    IntConversion(#[source] std::num::TryFromIntError, i64),
    #[error("Failed to convert RLE to its compressed version due to a type conversion error. Tried to convert '{1:?}' to u8 and failed.")]
    StrConversion(#[source] std::str::Utf8Error, Vec<u8>),
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::coco::{EncodedRle, Rle};
    use proptest::prelude::*;
    use rstest::rstest;

    prop_compose! {
        #[allow(clippy::unwrap_used)]
        fn generate_rle(max_value: u32, max_elts: usize)
            (counts in prop::collection::vec(1..max_value, 2..max_elts))
            (width in 1..counts.iter().sum(), sum in Just(counts.iter().sum::<u32>()), mut counts in Just(counts))
             -> Rle {
                let height = sum / width + 1;
                *counts.last_mut().unwrap() += width * height - sum;
                Rle { counts,
                      size: vec![width, height]
                }
            }
    }

    proptest! {
        #[test]
        fn rle_decode_inverts_encode(rle in generate_rle(50, 20)){
            let encoded_rle = EncodedRle::try_from(&rle).unwrap();
            let decoded_rle = Rle::from(&encoded_rle);
            prop_assert_eq!(decoded_rle, rle);
        }
    }

    #[rstest]
    #[case::square(&Rle {counts: vec![6, 1, 40, 4, 5, 4, 5, 4, 21], size: vec![9, 10]},
                     &EncodedRle {size: vec![9, 10], counts: "61X13mN000`0".to_string()})]
    #[case::test1(&Rle {counts: vec![245, 5, 35, 5, 35, 5, 35, 5, 35, 5, 1190], size: vec![40, 40]},
                  &EncodedRle {size: vec![40, 40], counts: "e75S10000000ST1".to_string()})]
    fn encode_rle(#[case] rle: &Rle, #[case] expected_encoded_rle: &EncodedRle) {
        let encoded_rle = EncodedRle::try_from(rle).unwrap();
        assert_eq!(&encoded_rle, expected_encoded_rle);
    }
}
