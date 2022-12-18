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
    // /// Split a COCO dataset in two.
    // Split(SplitCommand),
}

#[derive(Debug, Args)]
pub struct VisualizeCommand {
    #[clap(subcommand)]
    pub command: VisualizeSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum VisualizeSubcommand {
    /// Visualize a single sample.
    Sample(VisualizeSample),
    // /// Visualize all samples one by one.
    // All(VisualizeAll),
}

#[derive(Debug, Args)]
pub struct VisualizeSample {
    /// Path to the COCO json annotation file.
    pub annotation_file: String,

    /// Path to the folder with the images.
    pub image_folder: String,
}
