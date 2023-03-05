use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

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
        /// TODO
        #[arg(short, long)]
        sample_id: Option<u32>,
    },

    ConvertSegmentation {
        /// Path to the COCO json annotation file.
        annotations_file: PathBuf,
        target_segmentation: Segmentation,
        /// TODO
        #[arg(short, long)]
        output_folder: Option<PathBuf>,
    },
    // /// Split a COCO dataset in two.
    // Split(DatasetPathsArgs),
    // TODO: convert to/from PascalVOC, solo. Convert segmentation format.
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Segmentation {
    Polygon,
    Rle,
    EncodedRle,
}
