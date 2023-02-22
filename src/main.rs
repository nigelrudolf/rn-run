use std::env;
use clap::Parser;
use std::process::Command;


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
    let output = Command::new("lsof")
        .arg("-i")
        .arg(":8081")
        .arg("-t")
        .output()
        .expect("Failed to execute lsof command");

    let pids = String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .map(|pid| pid.parse::<u32>().expect("Failed to parse PID"))
        .collect::<Vec<u32>>();

    if pids.is_empty() {
        println!("No process running on port 8081");
    } else {
        for pid in pids {
            Command::new("kill")
                .arg(pid.to_string())
                .spawn()
                .expect("Failed to execute kill command");
        }
    }
}

fn quit_simulator() {
    Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Simulator\" to quit")
        .status()
        .expect("Failed to execute osascript command");
}

fn close_terminal_windows() {
    Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Terminal\" to close (every window)")
        .status()
        .expect("Failed to execute osascript command");
}

fn run_ios(simulator: Option<String>) {
    match env::current_dir() {
        Ok(watch_dir) => {
            println!("Watching directory: {:?}", watch_dir);
            // Do something with the watch_dir path
        }
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            // Handle the error case
        }
    }

    kill_process();
    quit_simulator();
    close_terminal_windows();

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