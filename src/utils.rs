use std::{env, process::Command, io::Error};
use crate::args::Args;

pub fn get_current_directory() -> Result<String, Error> {
    let current_dir = env::current_dir()
        .expect("Failed to get current directory")
        .to_str()
        .expect("Failed to convert current directory to string")
        .to_owned();

    println!("Current directory: {}", current_dir);

    Ok(current_dir)
}

pub fn kill_process() {
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

pub fn quit_simulator() {
    Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Simulator\" to quit")
        .status()
        .expect("Failed to execute osascript command");
}

pub fn close_terminal_windows() {
    Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Terminal\" to close (every window)")
        .status()
        .expect("Failed to execute osascript command");
}

pub fn clean_install() {
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

pub fn watch_directory(watch_dir: &Result<String, Error>) {
    
    let watch_dir_str = match watch_dir.as_ref() {
        Ok(s) => {
            println!("Watching directory: {}", s);
            s
        },
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    Command::new("watchman")
        .arg("watch-del")
        .arg(watch_dir_str)
        .status()
        .expect("Failed to execute watchman watch-del command");

    Command::new("watchman")
        .arg("watch-project")
        .arg(watch_dir_str)
        .status()
        .expect("Failed to execute watchman watch-project command");
}

pub fn launch_packager() {
    let current_dir = env::current_dir()
    .expect("Failed to get current directory")
    .to_str()
    .expect("Failed to convert current directory to string")
    .to_owned();

    Command::new("osascript")
        .arg("-e")
        .arg(format!(
            "tell application \"Terminal\" to do script \"cd {}; yarn start\"",
            current_dir))
        .status()
        .expect("Failed to execute osascript command");
}

pub fn launch_sim(args: &Args) {

    let command = if args.ios {
        "yarn react-native run-ios"
    } else if args.android {
        "yarn react-native run-android"
    } else {
        "echo \"No platform specified, use --help for more info\""
    };

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
            args.simulator.as_ref().unwrap_or(&command.to_string())
        ))
        .status()
        .expect("Failed to execute osascript command");
}