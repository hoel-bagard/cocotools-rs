use std::error;

use clap::Parser;

mod annotations;
mod argparse;
mod errors;
mod visualize;
use crate::annotations::load_coco::load_json;
use crate::argparse::{Cli, Commands, Segmentation};

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Visualize {
            annotations_file,
            image_folder,
            sample_id,
        } => {
            let dataset = load_json(annotations_file);
            if let Some(sample_id) = sample_id {
                visualize::visualize_img(&dataset, image_folder, *sample_id)?;
            } else {
                todo!()
            }
        }
        Commands::ConvertSegmentation {
            annotations_file,
            target_segmentation,
            output_folder,
        } => {
            println!("AAAAAAAAAAAAAAAAAAAAAAA");
        }
    }
    Ok(())
}
