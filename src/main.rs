mod annotations;
mod args;
mod visualize;

use args::COCOtoolsArgs;
use clap::Parser;

use crate::annotations::load_coco_annotations::load_json;
use crate::visualize::bbox;

fn main() {
    let args = COCOtoolsArgs::parse();

    println!("{:?}", args);

    match args.command_type {
        args::CommandType::Visualize(visualize_command) => match visualize_command.command {
            args::VisualizeSubcommand::VisualizeSample(sample_args) => {
                load_json(&sample_args.annotation_file);
                ()
            }
            args::VisualizeSubcommand::VisualizeAll(dataset_paths) => {
                bbox::draw_bbox();
                ()
            }
        },
        args::CommandType::Split(split_args) => (),
    };
}
