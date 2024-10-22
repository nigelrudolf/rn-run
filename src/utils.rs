use std::{env, fs::File, io::{self, Error, Read}, path::PathBuf, process::Command};
use crate::args::Args;
use serde::Deserialize;

pub fn get_current_directory() -> Result<String, Error> {
    let current_dir = env::current_dir()?
        .to_str()
        .ok_or_else(|| Error::new(std::io::ErrorKind::Other, "Failed to convert current directory to string"))?
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

#[derive(Deserialize)]
pub struct PackageJson {
    dependencies: Option<std::collections::HashMap<String, String>>,
}

pub fn get_react_native_version(path: &PathBuf) -> Result<Option<String>, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let package_json: PackageJson = serde_json::from_str(&contents)?;

    let version = package_json
        .dependencies
        .as_ref()
        .and_then(|deps| deps.get("react-native").cloned());
    
    Ok(version)
}

pub fn clean_install(react_native_version: &str) {

    let command = if is_version_greater_or_equal(react_native_version, "0.74") {
        "npm"
    } else if react_native_version.starts_with("0.69") {
        "yarn"
    } else {
        "yarn"
    };

    Command::new("rm")
        .arg("-rf")
        .arg("node_modules")
        .status()
        .expect("Failed to execute rm command");

    Command::new(command)
        .arg("install")
        .status()
        .expect("Failed to execute yarn command");

    Command::new("sh")
        .arg("-c")
        .arg("cd ios && pod install && cd ..")
        .status()
        .expect("Failed to execute shell command");   
}

pub fn watch_directory(watch_dir: &str) {
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

pub fn launch_sim(react_native_version: &str, args: &Args) {

    let yarn_ios = "yarn react-native run-ios";
    let yarn_android = "yarn react-native run-android";
    let npx_ios = format!("npm run prebuild && npx react-native run-ios --simulator=\"{}\"", args.simulator.as_ref().unwrap_or(&"iPhone 15".to_string()));
    let npx_android = "npm run prebuild && npx react-native run-android";

    let command = if args.ios && is_version_greater_or_equal(react_native_version, "0.74") {
        &npx_ios
    } else if args.ios && react_native_version.starts_with("0.69") {
        yarn_ios
    } else if args.android && is_version_greater_or_equal(react_native_version, "0.74") {
        npx_android
    } else if args.android && react_native_version.starts_with("0.69") {
        yarn_android
    } else {
        "echo \"No platform specified, use --help for more info\""
    };

    let current_dir = env::current_dir()
        .expect("Failed to get current directory")
        .to_str()
        .expect("Failed to convert current directory to string")
        .to_owned();

    let osascript_command = format!(
        "tell application \"Terminal\" to do script \"cd {}; {}\"",
        current_dir,
        command.replace("\"", "\\\"")
    );
  
    Command::new("osascript")
        .arg("-e")
        .arg(&osascript_command)
        .status()
        .expect("Failed to execute osascript command");
}


pub fn is_version_greater_or_equal(version: &str, target: &str) -> bool {
    // Split version and target into components
    let version_parts: Vec<&str> = version.split('.').collect();
    let target_parts: Vec<&str> = target.split('.').collect();

    // Compare major versions
    if version_parts[0] > target_parts[0] {
        return true;
    } else if version_parts[0] < target_parts[0] {
        return false;
    }

    // Compare minor versions if major versions are equal
    if version_parts[1] > target_parts[1] {
        return true;
    } else if version_parts[1] < target_parts[1] {
        return false;
    }

    // Compare patch versions if both major and minor are equal (optional)
    if version_parts.len() > 2 && target_parts.len() > 2 {
        if version_parts[2] >= target_parts[2] {
            return true;
        }
    }

    // If all parts are equal, the version is equal or greater
    true
}