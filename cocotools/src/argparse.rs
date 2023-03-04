use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Visualize COCO labels.
    Visualize {
        /// Path to the COCO json annotation file.
        annotations_file: PathBuf,

        /// Path to the folder with the images.
        image_folder: PathBuf,

        #[arg(short, long)]
        sample_id: Option<u32>,
    },
    // /// Split a COCO dataset in two.
    // Split(DatasetPathsArgs),
    // TODO: convert to/from PascalVOC, solo. Convert segmentation format.
}
