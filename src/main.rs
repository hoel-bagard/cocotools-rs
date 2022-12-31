mod annotations;
use crate::annotations::load_coco_annotations::load_json;

mod argparse;
mod visualize;

use clap::Parser;

fn main() {
    let args = argparse::COCOtoolsArgs::parse();

    match args.command_type {
        argparse::CommandType::Visualize(visualize_command) => match visualize_command.command {
            argparse::VisualizeSubcommand::VisualizeSample(sample_args) => {
                let dataset = load_json(&sample_args.annotations_file);
                visualize::visualize_sample(
                    &dataset,
                    &sample_args.image_folder,
                    sample_args.sample_id,
                );
            }
            argparse::VisualizeSubcommand::VisualizeAll(_dataset_paths) => (),
        },
        argparse::CommandType::Split(_split_args) => (),
    };
}
