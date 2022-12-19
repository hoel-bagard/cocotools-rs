mod args;
use std::path::Path;
mod annotations;

use args::COCOtoolsArgs;
use clap::Parser;

use crate::annotations::load_coco_annotations::load_json;

fn main() {
    let args = COCOtoolsArgs::parse();

    println!("{:?}", args);

    match args.command_type {
        args::CommandType::Visualize(visualize_command) => match visualize_command.command {
            args::VisualizeSubcommand::VisualizeSample(sample_args) => {
                load_json(&sample_args.annotation_file);
                1
            }
            args::VisualizeSubcommand::VisualizeAll(dataset_paths) => 1,
        },
        args::CommandType::Split(split_args) => 1,
    };
}
