use std::error;
use std::path::PathBuf;

use clap::Parser;

mod argparse;
mod coco;
mod errors;
mod mask;
mod utils;
mod visualize;
use crate::argparse::{Cli, Commands};
use crate::coco::COCO;
use crate::visualize::display;

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
                let img = dataset.draw_img_anns(*sample_id, true)?;
                display::img(&img, &dataset.get_img(*sample_id)?.file_name)?;
            } else {
                for img_entry in dataset.get_imgs() {
                    let img = dataset.draw_img_anns(img_entry.id, true)?;
                    display::img(&img, &img_entry.file_name)?;
                }
            }
        }
        Commands::ConvertSegmentation {
            annotations_path,
            target_segmentation,
            output_path,
        } => {
            let mut dataset = COCO::new(annotations_path, &PathBuf::from("N/A"))?;
            mask::conversions::convert_coco_segmentation(&mut dataset, *target_segmentation)?;
            let output_path = output_path
                .as_ref()
                .map_or_else(|| annotations_path, |output_path| output_path);
            dataset.save_to(output_path)?;
        }
    }
    Ok(())
}
