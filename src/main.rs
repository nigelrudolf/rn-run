mod args;
mod utils;
mod ios;
mod android;
mod main_test;
mod error;

use std::path::Path;

use clap::Parser;
use args::Args;
use ios::run_ios;
use android::run_android;
use utils::{get_current_directory, get_react_native_version};
use error::{AppError, Result};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    let current_dir = get_current_directory()?;
    let package_json_path = Path::new(&current_dir).join("package.json");

    let react_native_version = match get_react_native_version(&package_json_path)? {
        Some(version) => version,
        None => return Err(AppError::ReactNativeNotFound),
    };

    match args {
        Args { ios: true, android: false, .. } => run_ios(&args, &current_dir, &react_native_version),
        Args { android: true, ios: false, .. } => run_android(&args, &current_dir, &react_native_version),
        Args { ios: true, android: true, .. } => {
            eprintln!("Cannot run both iOS and Android simultaneously");
            std::process::exit(1);
        }
        _ => {
            println!("No platform specified, use --help for more info");
            Ok(())
        }
    }
}