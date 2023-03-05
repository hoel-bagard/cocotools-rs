use std::error;

use clap::Parser;

mod annotations;
mod argparse;
mod converters;
mod errors;
mod visualize;
use crate::annotations::coco;
use crate::argparse::{Cli, Commands};

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Visualize {
            annotations_file,
            image_folder,
            sample_id,
        } => {
            let dataset = coco::load_anns(annotations_file);
            if let Some(sample_id) = sample_id {
                visualize::visualize_img(&dataset, image_folder, *sample_id)?;
            } else {
                todo!()
            }
        }
        Commands::ConvertSegmentation {
            annotations_path,
            target_segmentation,
            output_folder,
        } => {
            let mut dataset = coco::load_anns(annotations_path);
            converters::masks::convert_coco_segmentation(&mut dataset, *target_segmentation);
            let output_path = output_folder.as_ref().map_or_else(
                || annotations_path.to_owned(),
                |output_folder| {
                    output_folder.to_owned().push("annotations.json");
                    output_folder.to_path_buf()
                },
            );
            coco::save_anns(output_path, dataset);
        }
    }
    Ok(())
}
