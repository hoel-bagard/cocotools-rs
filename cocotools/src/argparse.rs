use clap::{Args, Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct COCOtoolsArgs {
    #[clap(subcommand)]
    pub command_type: CommandType,
    // #[arg(long, short = 'v', action = clap::ArgAction::Count)]
    // verbose: i8,
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
    /// Visualize COCO labels.
    Visualize(VisualizeCommand),

    /// Split a COCO dataset in two.
    Split(DatasetPathsArgs),
    // TODO: convert to/from PascalVOC, solo. Convert segmentation format.
}

#[derive(Debug, Args)]
pub struct VisualizeCommand {
    #[clap(subcommand)]
    pub command: VisualizeSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum VisualizeSubcommand {
    /// Visualize a single sample.
    VisualizeSample(VisualizeSampleArgs),

    /// Visualize all samples one by one.
    VisualizeAll(DatasetPathsArgs),
}

#[derive(Debug, Args)]
pub struct VisualizeSampleArgs {
    /// Path to the COCO json annotation file.
    pub annotations_file: String,

    /// Path to the folder with the images.
    pub image_folder: String,

    pub sample_id: u32,
}

#[derive(Debug, Args)]
pub struct DatasetPathsArgs {
    /// Path to the COCO json annotation file.
    pub annotations_file: String,

    /// Path to the folder with the images.
    pub image_folder: String,
}
