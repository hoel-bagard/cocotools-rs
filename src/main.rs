mod args;

use args::COCOtoolsArgs;
use clap::Parser;

fn main() {
    let args = COCOtoolsArgs::parse();

    println!("{:?}", args);
}
