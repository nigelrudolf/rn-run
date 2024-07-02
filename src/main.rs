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
use utils::{get_current_directory, get_react_native_version};

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

    let react_native_version = match get_react_native_version(&package_json_path) {
        Ok(Some(version)) => version,
        Ok(None) => {
            eprintln!("React Native version not found in package.json");
            return;
        }
        Err(e) => {
            eprintln!("Failed to get React Native version: {}", e);
            return;
        }
    };

    match args {
        Args { ios: true, .. } => run_ios(&args, &current_dir, &react_native_version),
        Args { android: true, .. } => run_android(&args, &current_dir, &react_native_version),
        _ => println!("No platform specified, use --help for more info"),
    }
}