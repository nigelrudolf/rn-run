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

fn get_current_directory() -> String {
    let current_dir = env::current_dir()
        .expect("Failed to get current directory")
        .to_str()
        .expect("Failed to convert current directory to string")
        .to_owned();

    println!("Current directory: {}", current_dir);

    current_dir
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

fn clean_install() {
    Command::new("rm")
        .arg("-rf")
        .arg("node_modules")
        .status()
        .expect("Failed to execute rm command");

    Command::new("yarn")
        .arg("install")
        .status()
        .expect("Failed to execute yarn command");

    Command::new("sh")
        .arg("-c")
        .arg("cd ios && pod install && cd ..")
        .status()
        .expect("Failed to execute shell command");
}

fn watch_directory(watch_dir: &str) {
    println!("Watching directory: {}", watch_dir);

    Command::new("watchman")
        .arg("watch-del")
        .arg(watch_dir)
        .status()
        .expect("Failed to execute watchman watch-del command");

    Command::new("watchman")
        .arg("watch-project")
        .arg(watch_dir)
        .status()
        .expect("Failed to execute watchman watch-project command");
}

fn launch_sim(args: &Args) {
    let current_dir = env::current_dir()
        .expect("Failed to get current directory")
        .to_str()
        .expect("Failed to convert current directory to string")
        .to_owned();

    Command::new("osascript")
        .arg("-e")
        .arg(format!(
            "tell application \"Terminal\" to do script \"cd {}; {}\"",
            current_dir,
            args.simulator.as_ref().unwrap_or(&"yarn react-native run-ios".to_string())
        ))
        .status()
        .expect("Failed to execute osascript command");
}

fn run_ios(args: &Args) {
    let watch_dir = get_current_directory();

    kill_process();
    quit_simulator();
    close_terminal_windows();

    if args.clean {
        clean_install();
    }

    watch_directory(&watch_dir);

    launch_sim(args);
}

fn run_android() {
    println!("Running Android");
}

fn main() {
    let args = Args::parse();

    if args.ios {
        run_ios(&args);
    }

    if args.android {
        run_android();
    }

}