use image;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Dataset {
    pub images: Vec<Image>,
    pub annotations: Vec<Annotation>,
    pub categories: Vec<Category>,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub file_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Annotation {
    pub id: u32,
    pub image_id: u32,
    pub category_id: u32,
    /// Segmentation can be a polygon, RLE or encoded RLE.
    /// Exemple of polygon: "segmentation": [[510.66,423.01,511.72,420.03,...,510.45,423.01]]
    /// Exemple of RLE: "segmentation": {"size": [40, 40], "counts": [245, 5, 35, 5, 35, 5, 35, 5, 35, 5, 1190]}
    /// Exemple of encoded RLE: "segmentation": {"size": [480, 640], "counts": "aUh2b0X...BgRU4"}
    pub segmentation: Segmentation,
    pub area: f64,
    /// The COCO bounding box format is [top left x position, top left y position, width, height].
    /// bbox exemple:  "bbox": [473.07,395.93,38.65,28.67]
    pub bbox: Bbox,
    /// Either 1 or 0
    pub iscrowd: u32,
}

pub type Polygon = Vec<Vec<f64>>;

/// Internal type used to represent a polygon. It contains the width and height of the image for easier handling, notably when using traits.
#[derive(Deserialize, Debug)]
pub struct PolygonRS {
    pub size: Vec<u32>,
    pub counts: Vec<f64>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Segmentation {
    Polygon(Polygon),
    PolygonRS(PolygonRS),
    Rle(Rle),
    EncodedRle(EncodedRle),
}

/// TODO: Describe what size is.
#[derive(Deserialize, Debug)]
pub struct Rle {
    pub size: Vec<u32>,
    pub counts: Vec<u32>,
}

#[derive(Deserialize, Debug)]
pub struct EncodedRle {
    pub size: Vec<u32>,
    pub counts: String,
}

#[derive(Deserialize, Debug)]
pub struct Bbox {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub supercategory: String,
}

/// Decode encoded rle segmentation information into a rle.

/// See the (hard to read) implementation:
/// <https://github.com/cocodataset/cocoapi/blob/master/common/maskApi.c#L218>
/// <https://github.com/cocodataset/cocoapi/blob/8c9bcc3cf640524c4c20a9c40e89cb6a2f2fa0e9/PythonAPI/pycocotools/_mask.pyx#L145>

/// [LEB128 wikipedia article](https://en.wikipedia.org/wiki/LEB128#Decode_signed_integer)
/// It is similar to LEB128, but here shift is incremented by 5 instead of 7 because the implementation uses
/// 6 bits per byte instead of 8. (no idea why, I guess it's more efficient for the COCO dataset?)
#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
impl From<&EncodedRle> for Rle {
    /// Converts a compressed RLE to its uncompressed version.
    fn from(encoded_rle: &EncodedRle) -> Self {
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

        Self {
            size: encoded_rle.size.clone(),
            counts,
        }
    }
}

/// Convert a mask into its RLE form.
///
/// ## Args:
/// - mask: A binary mask indicating for each pixel whether it belongs to the object or not.
///
/// ## Returns:
/// - The RLE corresponding to the mask.
impl From<&image::GrayImage> for Rle {
    fn from(mask: &image::GrayImage) -> Self {
        let mut previous_value = 0;
        let mut count = 0;
        let mut counts = Vec::new();
        for pixel in mask.pixels() {
            if pixel[0] != previous_value {
                counts.push(count);
                previous_value = pixel[0];
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
