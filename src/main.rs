mod args;
mod utils;
mod ios;
mod android;

use clap::Parser;
use args::Args;
use ios::run_ios;
use android::run_android;

fn main() {
    let args = Args::parse();

    match args {
        Args { ios: true, .. } => run_ios(&args),
        Args { android: true, .. } => run_android(),
        _ => println!("No platform specified, use --help for more info"),
    }

}