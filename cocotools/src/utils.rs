// pub(crate) mod image;

use std::path::PathBuf;

use anyhow::{Context, Result};
use image::io::Reader as ImageReader;

use crate::errors::LoadingError;

/// Load an rgb8 image from the given path.
///
/// ## Args
/// - `img_path`: The path to the image to load.
///
/// ## Errors
///
/// Will return `Err` if the image could not be opened or decoded.
pub fn load_img(
    img_path: &PathBuf,
) -> Result<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, LoadingError> {
    Ok(ImageReader::open(img_path)
        .with_context(|| format!("Could not open the image `{}`.", img_path.display()))?
        .decode()
        .with_context(|| format!("Could not decode the image `{}`.", img_path.display()))?
        .into_rgb8())
}
