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
            let dataset = coco::load_anns(annotations_file)?;
            if let Some(sample_id) = sample_id {
                visualize::visualize_img(&dataset, image_folder, *sample_id)?;
            } else {
                for img_entry in dataset.get_imgs() {
                    let anns = dataset.get_img_anns(img_entry.id)?;
                    let img_path = image_folder.join(&img_entry.file_name);
                    visualize::show_anns(&img_path, anns, true)?;
                }
            }
        }
        Commands::ConvertSegmentation {
            annotations_path,
            target_segmentation,
            output_path,
        } => {
            let mut dataset = coco::load_anns(annotations_path)?;
            converters::masks::convert_coco_segmentation(&mut dataset, *target_segmentation)?;
            let output_path = output_path
                .as_ref()
                .map_or_else(|| annotations_path, |output_path| output_path);
            dataset.save_to(output_path)?;
        }
    }
    Ok(())
}
