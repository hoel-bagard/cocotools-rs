mod annotations;
use crate::annotations::load_coco_annotations::load_json;

mod args;
mod visualize;

use clap::Parser;

fn main() {
    let args = args::COCOtoolsArgs::parse();

    match args.command_type {
        args::CommandType::Visualize(visualize_command) => match visualize_command.command {
            args::VisualizeSubcommand::VisualizeSample(sample_args) => {
                let dataset = load_json(&sample_args.annotations_file);
                visualize::visualize_sample(
                    &dataset,
                    &sample_args.image_folder,
                    sample_args.sample_id,
                );
            }
            args::VisualizeSubcommand::VisualizeAll(_dataset_paths) => (),
        },
        args::CommandType::Split(_split_args) => (),
    };
}
