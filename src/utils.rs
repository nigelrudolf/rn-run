use std::{env, fs::File, io::Read, path::PathBuf, process::Command};
use crate::args::Args;
use crate::error::{AppError, Result};
use serde::Deserialize;

pub fn get_current_directory() -> Result<String> {
    let current_dir = env::current_dir()?
        .to_str()
        .ok_or(AppError::CurrentDir)?
        .to_owned();

    println!("Current directory: {}", current_dir);

    Ok(current_dir)
}

pub fn kill_process() -> Result<()> {
    let output = Command::new("lsof")
        .arg("-i")
        .arg(":8081")
        .arg("-t")
        .output()
        .map_err(|_| AppError::CommandFailed("lsof -i :8081 -t".to_string()))?;

    let pids_result: std::result::Result<Vec<u32>, _> = String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .map(|pid| pid.parse::<u32>())
        .collect();

    let pids = match pids_result {
        Ok(pids) => pids,
        Err(_) => {
            println!("No process running on port 8081");
            return Ok(());
        }
    };

    if pids.is_empty() {
        println!("No process running on port 8081");
    } else {
        for pid in pids {
            Command::new("kill")
                .arg(pid.to_string())
                .spawn()
                .map_err(|_| AppError::CommandFailed(format!("kill {}", pid)))?;
        }
    }
    
    Ok(())
}

pub fn quit_simulator() -> Result<()> {
    Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Simulator\" to quit")
        .status()
        .map_err(|_| AppError::CommandFailed("osascript quit simulator".to_string()))?;
    
    Ok(())
}

pub fn close_terminal_windows() -> Result<()> {
    Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Terminal\" to close (every window)")
        .status()
        .map_err(|_| AppError::CommandFailed("osascript close terminal windows".to_string()))?;
    
    Ok(())
}

#[derive(Deserialize)]
pub struct PackageJson {
    dependencies: Option<std::collections::HashMap<String, String>>,
}

pub fn get_react_native_version(path: &PathBuf) -> Result<Option<String>> {
    if !path.exists() {
        return Err(AppError::ReactNativeNotFound);
    }
    
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

pub fn clean_install(react_native_version: &str, platform: &str) -> Result<()> {

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
        .map_err(|_| AppError::CommandFailed("rm -rf node_modules".to_string()))?;

    Command::new("rm")
        .arg("-rf")
        .arg("package-lock.json")
        .status()
        .map_err(|_| AppError::CommandFailed("rm -rf package-lock.json".to_string()))?;


    Command::new(command)
        .arg("install")
        .status()
        .map_err(|_| AppError::CommandFailed(format!("{} install", command)))?;

    if platform == "ios" {
        Command::new("rm")
            .arg("-rf")
            .arg("ios/Pods")
            .status()
            .map_err(|_| AppError::CommandFailed("rm -rf ios/Pods".to_string()))?;

        Command::new("rm")
            .arg("-rf")
            .arg("ios/build")
            .status()
            .map_err(|_| AppError::CommandFailed("rm -rf ios/build".to_string()))?;

        Command::new("rm")
            .arg("-rf")
            .arg("ios/Podfile.lock")
            .status()
            .map_err(|_| AppError::CommandFailed("rm -rf ios/Podfile.lock".to_string()))?;

        Command::new("sh")
            .arg("-c")
            .arg("cd ios && pod install && cd ..")
            .status()
            .map_err(|_| AppError::CommandFailed("pod install".to_string()))?;
    } else if platform == "android" {
        Command::new("rm")
            .arg("-rf")
            .arg("android/build")
            .status()
            .map_err(|_| AppError::CommandFailed("rm -rf android/build".to_string()))?;

        Command::new("rm")
            .arg("-rf")
            .arg("android/app/build")
            .status()
            .map_err(|_| AppError::CommandFailed("rm -rf android/app/build".to_string()))?;

        Command::new("rm")
            .arg("-rf")
            .arg("android/.gradle")
            .status()
            .map_err(|_| AppError::CommandFailed("rm -rf android/.gradle".to_string()))?;
    }

    Ok(())
}

pub fn watch_directory(watch_dir: &str) -> Result<()> {
    println!("Watching directory: {}", watch_dir);

    Command::new("watchman")
        .arg("watch-del")
        .arg(watch_dir)
        .status()
        .map_err(|_| AppError::CommandFailed("watchman watch-del".to_string()))?;

    Command::new("watchman")
        .arg("watch-project")
        .arg(watch_dir)
        .status()
        .map_err(|_| AppError::CommandFailed("watchman watch-project".to_string()))?;
    
    Ok(())
}

pub fn launch_packager() -> Result<()> {
    let current_dir = env::current_dir()
        .map_err(|_| AppError::CurrentDir)?
        .to_str()
        .ok_or(AppError::CurrentDir)?
        .to_owned();

    Command::new("osascript")
        .arg("-e")
        .arg(format!(
            "tell application \"Terminal\" to do script \"cd {}; yarn start\"",
            current_dir))
        .status()
        .map_err(|_| AppError::CommandFailed("launch packager".to_string()))?;
    
    Ok(())
}

pub fn launch_sim(react_native_version: &str, args: &Args) -> Result<()> {

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
        .map_err(|_| AppError::CurrentDir)?
        .to_str()
        .ok_or(AppError::CurrentDir)?
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
        .map_err(|_| AppError::CommandFailed("launch simulator".to_string()))?;
    
    Ok(())
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