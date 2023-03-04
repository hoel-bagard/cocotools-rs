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

// From https://www.lpalmieri.com/posts/error-handling-rust/
//      https://github.com/LukeMathWalker/zero-to-production/blob/main/src/routes/subscriptions.rs#L199
impl std::fmt::Debug for MissingIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
