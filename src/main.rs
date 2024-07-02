mod args;
mod utils;
mod ios;
mod android;
mod main_test;

use std::path::Path;

use clap::Parser;
use args::Args;
use ios::run_ios;
use android::run_android;
use utils::get_current_directory;

fn main() {
    let args = Args::parse();

    let current_dir = match get_current_directory() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            return;
        }
    };

    let package_json_path = Path::new(&current_dir).join("package.json");

    match args {
        Args { ios: true, .. } => run_ios(&args, &current_dir, &package_json_path),
        Args { android: true, .. } => run_android(&args, &current_dir, &package_json_path),
        _ => println!("No platform specified, use --help for more info"),
    }
}