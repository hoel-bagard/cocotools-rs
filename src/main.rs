mod annotations;
mod argparse;
mod errors;
mod visualize;
use crate::annotations::load_coco::load_json;

use clap::Parser;

fn main() {
    let args = argparse::COCOtoolsArgs::parse();

    match args.command_type {
        argparse::CommandType::Visualize(visualize_command) => match visualize_command.command {
            argparse::VisualizeSubcommand::VisualizeSample(sample_args) => {
                let dataset = load_json(&sample_args.annotations_file);
                match visualize::visualize_img(
                    &dataset,
                    &sample_args.image_folder,
                    sample_args.sample_id,
                ) {
                    Ok(()) => {}
                    Err(err) => {
                        println!("{err}")
                    }
                }
            }
            argparse::VisualizeSubcommand::VisualizeAll(_dataset_paths) => (),
        },
        argparse::CommandType::Split(_split_args) => (),
    };
}
