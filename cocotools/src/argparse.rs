use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::mask::conversions::Segmentation;

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
        /// The id of the image to visualize. It is often the same as the filename, but not necessarily.
        #[arg(short, long)]
        sample_id: Option<u32>,
    },

    /// Convert the segmentation format of the labels in a COCO annotation file.
    ConvertSegmentation {
        /// Path to the COCO json annotation file.
        annotations_path: PathBuf,
        target_segmentation: Segmentation,
        /// Path to where the output will be saved (for example "output/annotation_rle.json"). If not given, the conversion is done in place.
        #[arg(short, long)]
        output_path: Option<PathBuf>,
    },
    // Split a COCO dataset in two.
    // Convert to/from PascalVOC, SOLO.
}
