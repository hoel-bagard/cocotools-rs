use clap::ValueEnum;
use image;
use imageproc::contours;
use imageproc::drawing;
use ndarray::{s, Array2, ArrayViewMut, ShapeBuilder};

use crate::annotations::coco;
use crate::errors::MaskError;

/// A boolean mask indicating for each pixel whether it belongs to the object or not.
pub type Mask = Array2<u8>;

/// Segmentation types.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Segmentation {
    Polygons,
    Rle,
    EncodedRle,
}

/// Converts all the segmentation masks in the dataset to the desired type.
///
/// # Errors
///
/// Will return `Err` if the conversion failed.
pub fn convert_coco_segmentation(
    dataset: &mut coco::HashmapDataset,
    target_segmentation: Segmentation,
) -> Result<(), MaskError> {
    for ann in dataset.anns.values_mut() {
        let converted_segmentation = match &ann.segmentation {
            coco::Segmentation::Rle(rle) => match target_segmentation {
                Segmentation::Rle => coco::Segmentation::Rle(rle.clone()),
                Segmentation::EncodedRle => {
                    coco::Segmentation::EncodedRle(coco::EncodedRle::try_from(rle)?)
                }
                Segmentation::Polygons => coco::Segmentation::Polygons(coco::Polygons::from(rle)),
            },
            coco::Segmentation::EncodedRle(encoded_rle) => match target_segmentation {
                Segmentation::Rle => coco::Segmentation::Rle(coco::Rle::from(encoded_rle)),
                Segmentation::EncodedRle => coco::Segmentation::EncodedRle(encoded_rle.clone()),
                Segmentation::Polygons => coco::Segmentation::Polygons(coco::Polygons::from(
                    &coco::Rle::from(encoded_rle),
                )),
            },
            coco::Segmentation::PolygonsRS(poly) => match target_segmentation {
                Segmentation::Rle => coco::Segmentation::Rle(coco::Rle::try_from(poly)?),
                Segmentation::EncodedRle => coco::Segmentation::EncodedRle(
                    coco::EncodedRle::try_from(&coco::Rle::from(&Mask::try_from(poly)?))?,
                ),
                Segmentation::Polygons => coco::Segmentation::Polygons(poly.counts.clone()),
            },
            coco::Segmentation::Polygons(_) => unimplemented!(),
        };
        ann.segmentation = converted_segmentation;
    }
    Ok(())
}

#[allow(clippy::expect_used)]
impl From<&coco::Rle> for coco::Polygons {
    fn from(rle: &coco::Rle) -> Self {
        let mask = Mask::from(rle);
        let mask_img = mask
            .as_slice_memory_order()
            .map(|slice| {
                image::GrayImage::from_raw(rle.size[1], rle.size[0], slice.to_owned()).expect(
                    "Buffer already contains a mask created using the rle sizes and is threfore big enough."
                )
            })
            .expect("The mask is created just above and should therefore be continuous in memory.");

        let contours = contours::find_contours::<u32>(&mask_img);

        // find_contours returns all the points defining the contours, the following for loop removes all the points formings lines as they are not needed.
        let mut counts: Self = Self::new();
        let mut prev_prev_x: u32;
        let mut prev_prev_y: u32;
        let mut prev_x: u32;
        let mut prev_y: u32;
        for (i, contour) in contours.iter().enumerate() {
            // Valid polygons must have at least 3 points.
            // The case of having less than 3 points is not expected to occur on real data, hence the silent failt if it occurs.
            if contour.points.len() > 3 {
                counts.push(Vec::with_capacity(2 * contour.points.len()));

                counts[i].push(f64::from(contour.points[0].y));
                counts[i].push(f64::from(contour.points[0].x));
                prev_prev_x = contour.points[0].x;
                prev_prev_y = contour.points[0].y;
                prev_x = contour.points[1].x;
                prev_y = contour.points[1].y;
                for point in &contour.points {
                    if !((prev_prev_x == prev_x && prev_x == point.x)
                        || (prev_prev_y == prev_y && prev_y == point.y))
                    {
                        counts[i].push(f64::from(prev_y));
                        counts[i].push(f64::from(prev_x));
                    }
                    prev_prev_x = prev_x;
                    prev_prev_y = prev_y;
                    prev_x = point.x;
                    prev_y = point.y;
                }

                if !((prev_prev_x == prev_x && prev_x == contour.points[0].x)
                    || (prev_prev_y == prev_y && prev_y == contour.points[0].y))
                {
                    counts[i].push(f64::from(prev_y));
                    counts[i].push(f64::from(prev_x));
                }
            }
        }
        counts
    }
}

impl TryFrom<&coco::PolygonsRS> for coco::Rle {
    type Error = MaskError;
    // It might be more efficient to do it like this: https://github.com/cocodataset/cocoapi/blob/master/common/maskApi.c#L162
    // It would also avoid having slightly different results from the reference implementation.
    fn try_from(poly: &coco::PolygonsRS) -> Result<Self, Self::Error> {
        Ok(Self::from(&Mask::try_from(poly)?))
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
                // "vertically convex" (i.e. have only one continuous run per line).
                // In which case, the next "row" of black/white pixels is going to be similar to the one preceding it.
                // Therefore, by having the continuous count of pixels be an offset of the one preceding it, we can have it be
                // a smaller int and therefore use less bits to encode it.
                continuous_pixels += counts[current_count_idx - 2] as i32;
            }
            counts[current_count_idx] = continuous_pixels as u32;
            current_count_idx += 1;
        }

        // Added the while loop to make it work, but it should not be there. Something is wrong somewhere else.
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

#[allow(clippy::expect_used)]
impl From<&coco::Rle> for Mask {
    /// Converts a RLE to its uncompressed mask.
    #[allow(clippy::cast_possible_truncation)]
    fn from(rle: &coco::Rle) -> Self {
        let width = rle.size[1] as usize;
        let height = rle.size[0] as usize;

        let mut mask: Self = Self::zeros((height, width).f());
        let mut mask_1d = ArrayViewMut::from_shape(
            (height * width).f(),
            mask.as_slice_memory_order_mut().expect("The mask array is created just above, there shouldn't be any error when creating a view of it"),
        )
        .expect("The mask array is created just above, there shouldn't be any error when creating a view of it");

        let mut current_value = 0u8;
        let mut current_position = 0usize;
        for nb_pixels in &rle.counts {
            mask_1d
                .slice_mut(s![current_position..current_position + *nb_pixels as usize])
                .fill(current_value);
            current_value = u8::from(current_value == 0);
            current_position += *nb_pixels as usize;
        }
        mask
    }
}

/// Convert a mask into its RLE form.
///
/// ## Args:
/// - mask: A binary mask indicating for each pixel whether it belongs to the object or not.
///
/// ## Returns:
/// - The RLE corresponding to the mask.
// The implementation makes a clone of the mask, which is expensive. This could be avoided by taking a mutable reference and reversing the axes again after the for loop.
// However asking for a mutable reference might be confusing.
#[allow(clippy::cast_possible_truncation)]
impl From<&Mask> for coco::Rle {
    fn from(mask: &Mask) -> Self {
        let mut previous_value = 0;
        let mut count = 0;
        let mut counts = Vec::new();
        for value in mask.clone().reversed_axes().iter() {
            if *value != previous_value {
                counts.push(count);
                previous_value = *value;
                count = 0;
            }
            count += 1;
        }
        counts.push(count);

        Self {
            size: vec![mask.nrows() as u32, mask.ncols() as u32],
            counts,
        }
    }
}

impl TryFrom<&coco::Segmentation> for Mask {
    type Error = MaskError;

    fn try_from(coco_segmentation: &coco::Segmentation) -> Result<Self, Self::Error> {
        let mask = match coco_segmentation {
            coco::Segmentation::Rle(rle) => Self::from(rle),
            coco::Segmentation::EncodedRle(encoded_rle) => {
                Self::from(&coco::Rle::from(encoded_rle))
            }
            coco::Segmentation::PolygonsRS(poly) => Self::try_from(poly)?,
            coco::Segmentation::Polygons(_) => {
                unimplemented!("Use the 'mask_from_poly' function.")
            }
        };
        Ok(mask)
    }
}

#[allow(clippy::cast_possible_truncation)]
impl TryFrom<&coco::PolygonsRS> for Mask {
    type Error = MaskError;

    /// Create a mask from a compressed polygon representation.
    fn try_from(poly_ann: &coco::PolygonsRS) -> Result<Self, Self::Error> {
        let mut mask = image::GrayImage::new(poly_ann.size[1], poly_ann.size[0]);

        for poly in &poly_ann.counts {
            let mut points_poly: Vec<imageproc::point::Point<i32>> = Vec::new();
            for i in (0..poly.len()).step_by(2) {
                points_poly.push(imageproc::point::Point::new(
                    poly[i] as i32,
                    poly[i + 1] as i32,
                ));
            }
            if let Some(last_point) = points_poly.last() {
                if points_poly[0].x == last_point.x && points_poly[0].y == last_point.y {
                    points_poly.pop();
                }
            }

            drawing::draw_polygon_mut(&mut mask, &points_poly, image::Luma([1u8]));
        }

        Self::from_shape_vec(
            (poly_ann.size[1] as usize, poly_ann.size[0] as usize),
            mask.into_raw(),
        )
        .map_err(MaskError::ImageToNDArrayConversion)
    }
}

/// Decompress a polygon representation of a mask.
///
/// ## Args:
/// - poly: A mask compressed as a COCO polygon.
/// - width: The original width of the image the polygon annotation corresponds to.
/// - height: The original height of the image the polygon annotation corresponds to.
///
/// ## Errors
/// Will return `Err` if the internal conversion from `ImageBuffer` to Mask (ndarray) fails.
///
/// ## Returns:
/// - The decompressed mask.
#[allow(clippy::cast_possible_truncation, clippy::module_name_repetitions)]
pub fn mask_from_poly(poly: &coco::Polygons, width: u32, height: u32) -> Result<Mask, MaskError> {
    let mut points_poly: Vec<imageproc::point::Point<i32>> = Vec::new();
    for i in (0..poly[0].len()).step_by(2) {
        points_poly.push(imageproc::point::Point::new(
            poly[0][i] as i32,
            poly[0][i + 1] as i32,
        ));
    }
    let mut mask = image::GrayImage::new(width, height);
    drawing::draw_polygon_mut(&mut mask, &points_poly, image::Luma([1u8]));

    Mask::from_shape_vec((height as usize, width as usize), mask.into_raw())
        .map_err(MaskError::ImageToNDArrayConversion)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::coco::{EncodedRle, Polygons, PolygonsRS, Rle};
    use super::*;
    use ndarray::array;
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

    prop_compose! {
        fn generate_mask(max_ncols: usize, max_nrows: usize)
            (ncols in 2..max_ncols, nrows in 2..max_nrows)
            (ncols in Just(ncols),
             nrows in Just(nrows),
             mask_data in prop::collection::vec(0..=1u8, ncols * nrows),
            ) -> Mask {
                Mask::from_shape_vec((nrows, ncols), mask_data).unwrap()
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

    proptest! {
        #[test]
        fn mask_to_rle_to_mask(mask in generate_mask(100, 100)){
            let rle = Rle::from(&mask);
            let decoded_mask = Mask::from(&rle);
            prop_assert_eq!(decoded_mask, mask);
        }
    }

    #[rstest]
    #[case::square(&Rle {size: vec![4, 4], counts: vec![5, 2, 2, 2, 5]})]
    #[case::thick_horizontal_line(&Rle { size: vec![7, 7], counts: vec![9, 3, 4, 3, 4, 3, 4, 3, 4, 3, 9] })]
    #[case::vertical_line(&Rle { size: vec![7, 7], counts: vec![15, 5, 2, 5, 2, 5, 15] })]
    fn rle_to_poly_to_rle(#[case] rle: &Rle) {
        let poly = Polygons::from(rle);
        let mask = mask_from_poly(&poly, rle.size[1], rle.size[0]).unwrap();
        let result_rle = Rle::try_from(&mask).unwrap();
        assert_eq!(&result_rle, rle);
    }

    #[rstest]
    #[case::square(
        &Rle {size: vec![4, 4], counts: vec![5, 2, 2, 2, 5]},
        &PolygonsRS {size: vec![4, 4], counts: vec![vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 1.0]] }
    )]
    #[case::horizontal_thick_line(
        &Rle {size: vec![7, 7], counts: vec![9, 3, 4, 3, 4, 3, 4, 3, 4, 3, 9]},
        &PolygonsRS {size: vec![7, 7], counts: vec![vec![1.0, 2.0, 1.0, 4.0, 5.0, 4.0, 5.0, 2.0]]}
    )]
    #[case::vertical_thick_line(
        &Rle {size: vec![7, 7], counts: vec![15, 5, 2, 5, 2, 5, 15]},
        &PolygonsRS {size: vec![7, 7], counts: vec![vec![2.0, 1.0, 2.0, 5.0, 4.0, 5.0, 4.0, 1.0]]}
    )]
    // There is no method defined for testing the equality of two polygons, the assert_eq is therefore done between PolygonsRS.
    fn rle_to_poly(#[case] rle: &Rle, #[case] expected_polygon: &PolygonsRS) {
        let poly = PolygonsRS {
            size: rle.size.clone(),
            counts: Polygons::from(rle),
        };
        assert_eq!(&poly, expected_polygon);
    }

    #[rstest]
    #[case::square(
        &array![[0, 0, 0, 0],
                [0, 1, 1, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0]],
        &Rle {size: vec![4, 4], counts: vec![5, 2, 2, 2, 5]})]
    #[case::horizontal_line(
        &array![[0, 0, 0, 0, 0],
                [1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0]],
        &Rle {size: vec![4, 5], counts: vec![1, 1, 3, 1, 3, 1, 3, 1, 3, 1, 2]})]
    #[case::vertical_line(
        &array![[0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0]],
        &Rle {size: vec![4, 5], counts: vec![8, 4, 8]})]
    #[case::thick_horizontal_line(
        &array![[0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 1, 1, 1, 1, 1, 0],
                [0, 1, 1, 1, 1, 1, 0],
                [0, 1, 1, 1, 1, 1, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0]],
        &Rle { size: vec![7, 7], counts: vec![9, 3, 4, 3, 4, 3, 4, 3, 4, 3, 9] })]
    #[case::thick_vertical_line(
        &array![[0, 0, 0, 0, 0, 0, 0],
                [0, 0, 1, 1, 1, 0, 0],
                [0, 0, 1, 1, 1, 0, 0],
                [0, 0, 1, 1, 1, 0, 0],
                [0, 0, 1, 1, 1, 0, 0],
                [0, 0, 1, 1, 1, 0, 0],
                [0, 0, 0, 0, 0, 0, 0]],
        &Rle { size: vec![7, 7], counts: vec![15, 5, 2, 5, 2, 5, 15] })]
    fn mask_to_rle(#[case] mask: &Mask, #[case] expected_rle: &Rle) {
        let rle = Rle::from(mask);
        assert_eq!(&rle, expected_rle);
    }

    #[rstest]
    #[case::square(
        &array![[0, 0, 0, 0],
                [0, 1, 1, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0]],
        &Rle {size: vec![4, 4], counts: vec![5, 2, 2, 2, 5]})]
    #[case::horizontal_line(
        &array![[0, 0, 0, 0, 0],
                [1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0]],
        &Rle {size: vec![4, 5], counts: vec![1, 1, 3, 1, 3, 1, 3, 1, 3, 1, 2]})]
    #[case::vertical_line(
        &array![[0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0]],
        &Rle {size: vec![4, 5], counts: vec![8, 4, 8]})]
    fn rle_to_mask(#[case] expected_mask: &Mask, #[case] rle: &Rle) {
        let mask = Mask::from(rle);
        assert_eq!(&mask, expected_mask);
    }

    #[rstest]
    #[case::square(
        &Rle {size: vec![4, 4], counts: vec![5, 2, 2, 2, 5]},
        &EncodedRle { size: vec![4, 4], counts: "52203".to_string() })]
    #[case::square2(
        &Rle {counts: vec![6, 1, 40, 4, 5, 4, 5, 4, 21], size: vec![9, 10]},
        &EncodedRle {size: vec![9, 10], counts: "61X13mN000`0".to_string()})]
    #[case::test1(
        &Rle {counts: vec![245, 5, 35, 5, 35, 5, 35, 5, 35, 5, 1190], size: vec![40, 40]},
        &EncodedRle {size: vec![40, 40], counts: "e75S10000000ST1".to_string()})]
    fn encode_rle(#[case] rle: &Rle, #[case] expected_encoded_rle: &EncodedRle) {
        let encoded_rle = EncodedRle::try_from(rle).unwrap();
        assert_eq!(&encoded_rle, expected_encoded_rle);
    }
}
