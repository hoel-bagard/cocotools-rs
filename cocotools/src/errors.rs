//! Module with the errors for the crate.
use std::path::PathBuf;

use thiserror::Error;

/// Error returned when trying to access an element of the dataset that does not exist.
#[derive(thiserror::Error)]
pub enum MissingIdError {
    #[error("The following annotation id was not found in the dataset: `{0}`.")]
    Annotation(u32),
    #[error("The following category id was not found in the dataset: `{0}`.")]
    Category(u32),
    #[error("The following image id was not found in the dataset: `{0}`.")]
    Image(u32),
    // #[error(transparent)]
    // InvalidValue(#[from] anyhow::Error),
}

/// Error returned when a json annotations file cannot be loaded/parsed or when an image cannot be loaded.
#[derive(Error)]
pub enum LoadingError {
    #[error("Failed to read the annotation file {1:?}.")]
    Read(#[source] std::io::Error, PathBuf),
    #[error("Failed to deserialize the annotation file {1:?}.")]
    Deserialize(#[source] serde_json::Error, PathBuf),
    #[error("Failed to parse the annotation file {1:?}. Found an annotation for an image id not in the dataset.")]
    Parsing(#[source] MissingIdError, PathBuf),
    #[error(transparent)]
    Image(#[from] anyhow::Error),
}

/// Error returned converting a segmentation mask to another format fails.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum MaskError {
    #[error("Failed to convert RLE to its compressed version due to a type conversion error. Tried to convert '{1:?}' to u8 and failed.")]
    IntConversion(#[source] std::num::TryFromIntError, i64),
    #[error("Failed to convert RLE to its compressed version due to a type conversion error. Tried to convert '{1:?}' to u8 and failed.")]
    StrConversion(#[source] std::str::Utf8Error, Vec<u8>),
    #[error("Failed to convert an image mask to an ndarray version of it.")]
    ImageToNDArrayConversion(#[source] ndarray::ShapeError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Enum grouping all the error types from the crate.
#[derive(Debug, Error)]
pub enum CocoError {
    #[error(transparent)]
    MissingId(#[from] MissingIdError),
    #[error(transparent)]
    Loading(#[from] LoadingError),
    #[error(transparent)]
    Mask(#[from] MaskError),
}

// From https://www.lpalmieri.com/posts/error-handling-rust/
//      https://github.com/LukeMathWalker/zero-to-production/blob/main/src/routes/subscriptions.rs#L199
impl std::fmt::Debug for MissingIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
impl std::fmt::Debug for LoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{e}\n")?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{cause}")?;
        current = cause.source();
    }
    Ok(())
}
