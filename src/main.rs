use std::env;
use clap::Parser;

/// Run react native app on ios or android
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run iOS
    #[arg(short, long)]
    ios: bool,

    /// Run Android
    #[arg(short, long)]
    android: bool,

    // Simulator
    #[arg(short, long)]
    simulator: Option<String>,

    /// Clean install
    #[arg(short, long)]
    clean: bool,
}

fn kill_process() {
    println!("Killing process");
}

fn run_ios(simulator: Option<String>) {
    match env::current_dir() {
        Ok(watch_dir) => {
            println!("Watch directory: {:?}", watch_dir);
            // Do something with the watch_dir path
        }
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            // Handle the error case
        }
    }

    kill_process();

    println!("Running iOS");
    match simulator {
        Some(x) => { println!("with simulator: {}", x); }
        None => { /* handle the None case */ }
    }
}

fn run_android() {
    println!("Running Android");
}

fn main() {
    let args = Args::parse();

    if args.ios {
        run_ios(args.simulator);
    }

    if args.android {
        run_android();
    }

}