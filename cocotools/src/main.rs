use std::error;
use std::path::PathBuf;

use clap::Parser;

mod annotations;
mod argparse;
mod converters;
mod errors;
mod visualize;
use crate::annotations::COCO;
use crate::argparse::{Cli, Commands};

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Visualize {
            annotations_file,
            image_folder,
            sample_id,
        } => {
            let dataset = COCO::new(annotations_file, image_folder)?;
            if let Some(sample_id) = sample_id {
                visualize::display::show_img_anns(&dataset, image_folder, *sample_id)?;
            } else {
                for img_entry in dataset.get_imgs() {
                    let anns = dataset.get_img_anns(img_entry.id)?;
                    let img_path = image_folder.join(&img_entry.file_name);
                    visualize::show_anns(&img_path, &anns, true)?;
                }
            }
        }
        Commands::ConvertSegmentation {
            annotations_path,
            target_segmentation,
            output_path,
        } => {
            let mut dataset = COCO::new(annotations_path, &PathBuf::from("N/A"))?;
            converters::masks::convert_coco_segmentation(&mut dataset, *target_segmentation)?;
            let output_path = output_path
                .as_ref()
                .map_or_else(|| annotations_path, |output_path| output_path);
            dataset.save_to(output_path)?;
        }
    }
    Ok(())
}
