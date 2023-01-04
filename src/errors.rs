use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MissingIdError {
    Annotation(MissingAnnotationIdError),
    Category(MissingCategoryIdError),
    Image(MissingImageIdError),
}

#[derive(Debug)]
pub struct MissingAnnotationIdError {
    pub id: u32,
}
#[derive(Debug)]
pub struct MissingCategoryIdError {
    pub id: u32,
}
#[derive(Debug)]
pub struct MissingImageIdError {
    pub id: u32,
}

impl fmt::Display for MissingIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MissingIdError::Annotation(ann_error) => write!(
                f,
                "The following annotation id was not found in the dataset: {}",
                ann_error.id
            ),
            MissingIdError::Category(cat_error) => write!(
                f,
                "The following category id was not found in the dataset: {}",
                cat_error.id
            ),
            MissingIdError::Image(img_error) => write!(
                f,
                "The following image id was not found in the dataset: {}",
                img_error.id
            ),
        }
    }
}

impl From<MissingAnnotationIdError> for MissingIdError {
    fn from(err: MissingAnnotationIdError) -> Self {
        MissingIdError::Annotation(err)
    }
}

impl From<MissingCategoryIdError> for MissingIdError {
    fn from(err: MissingCategoryIdError) -> Self {
        MissingIdError::Category(err)
    }
}

impl From<MissingImageIdError> for MissingIdError {
    fn from(err: MissingImageIdError) -> Self {
        MissingIdError::Image(err)
    }
}
impl Error for MissingIdError {}
