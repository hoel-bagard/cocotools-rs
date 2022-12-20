mod annotations;

mod args;
mod visualize;

use args::COCOtoolsArgs;
use clap::Parser;

fn main() {
    let args = COCOtoolsArgs::parse();

    println!("{:?}", args);

    match args.command_type {
        args::CommandType::Visualize(visualize_command) => match visualize_command.command {
            args::VisualizeSubcommand::VisualizeSample(sample_args) => {
                visualize::visualize_sample(sample_args);
                ()
            }
            args::VisualizeSubcommand::VisualizeAll(_dataset_paths) => (),
        },
        args::CommandType::Split(_split_args) => (),
    };
}
