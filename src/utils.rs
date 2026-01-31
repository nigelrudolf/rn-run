use std::{env, fs, fs::File, io::Read, path::PathBuf, process::Command, process::Stdio};
use crate::args::Args;
use crate::error::{AppError, Result};
use serde::Deserialize;
use chrono::Local;

const LOG_DIR: &str = ".rn-run/logs";
const MAX_LOGS: usize = 10;

pub fn get_current_directory_logged(log: Option<&LogWriter>) -> Result<String> {
    let current_dir = env::current_dir()?
        .to_str()
        .ok_or(AppError::CurrentDir)?
        .to_owned();

    let msg = format!("Current directory: {}", current_dir);
    if let Some(log) = log {
        log.log(&msg);
    } else {
        println!("{}", msg);
    }

    Ok(current_dir)
}

pub fn get_current_directory() -> Result<String> {
    get_current_directory_logged(None)
}

pub fn kill_process_logged(log: Option<&LogWriter>) -> Result<()> {
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
            let msg = "No process running on port 8081";
            if let Some(log) = log {
                log.log(msg);
            } else {
                println!("{}", msg);
            }
            return Ok(());
        }
    };

    if pids.is_empty() {
        let msg = "No process running on port 8081";
        if let Some(log) = log {
            log.log(msg);
        } else {
            println!("{}", msg);
        }
    } else {
        let count = pids.len();
        for pid in pids {
            Command::new("kill")
                .arg(pid.to_string())
                .spawn()
                .map_err(|_| AppError::CommandFailed(format!("kill {}", pid)))?;
        }
        let msg = format!("Killed {} Metro process(es) on port 8081", count);
        if let Some(log) = log {
            log.log(&msg);
        } else {
            println!("{}", msg);
        }
    }

    Ok(())
}

pub fn kill_process() -> Result<()> {
    kill_process_logged(None)
}

pub fn quit_simulator() -> Result<()> {
    Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Simulator\" to quit")
        .status()
        .map_err(|_| AppError::CommandFailed("osascript quit simulator".to_string()))?;

    Ok(())
}

pub fn take_ios_screenshot(output_path: Option<&str>) -> Result<String> {
    let path = match output_path {
        Some(p) => p.to_string(),
        None => {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            format!("screenshot-ios-{}.png", timestamp)
        }
    };

    let output = Command::new("xcrun")
        .args(["simctl", "io", "booted", "screenshot", &path])
        .output()
        .map_err(|_| AppError::CommandFailed("xcrun simctl io booted screenshot".to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::CommandFailed(format!(
            "Screenshot failed: {}. Is an iOS simulator running?",
            stderr.trim()
        )));
    }

    Ok(path)
}

pub fn take_android_screenshot(output_path: Option<&str>) -> Result<String> {
    let path = match output_path {
        Some(p) => p.to_string(),
        None => {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            format!("screenshot-android-{}.png", timestamp)
        }
    };

    // Use adb to capture screenshot and pull to local path
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "adb exec-out screencap -p > \"{}\"",
            path.replace("\"", "\\\"")
        ))
        .output()
        .map_err(|_| AppError::CommandFailed("adb exec-out screencap".to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::CommandFailed(format!(
            "Screenshot failed: {}. Is an Android emulator/device connected?",
            stderr.trim()
        )));
    }

    // Verify file was created and has content
    let metadata = std::fs::metadata(&path);
    if metadata.is_err() || metadata.unwrap().len() == 0 {
        return Err(AppError::CommandFailed(
            "Screenshot failed: No device connected or screenshot capture failed".to_string()
        ));
    }

    Ok(path)
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
    scripts: Option<std::collections::HashMap<String, String>>,
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

pub fn has_prebuild_script(path: &PathBuf) -> bool {
    if !path.exists() {
        return false;
    }

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return false;
    }

    let package_json: PackageJson = match serde_json::from_str(&contents) {
        Ok(p) => p,
        Err(_) => return false,
    };

    package_json
        .scripts
        .as_ref()
        .map(|s| s.contains_key("prebuild"))
        .unwrap_or(false)
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
    println!("\x1b[32m[rn-run]: node_modules deleted\x1b[0m");

    Command::new(command)
        .arg("install")
        .status()
        .map_err(|_| AppError::CommandFailed(format!("{} install", command)))?;
    println!("\x1b[32m[rn-run]: {} install completed\x1b[0m", command);

    if platform == "ios" {
        Command::new("sh")
            .arg("-c")
            .arg("cd ios && pod install && cd ..")
            .status()
            .map_err(|_| AppError::CommandFailed("pod install".to_string()))?;
        println!("\x1b[32m[rn-run]: pod install completed\x1b[0m");
    }

    Ok(())
}

pub fn deep_clean(platform: &str) -> Result<()> {
    println!("\x1b[32m[rn-run]: starting deep clean for {}\x1b[0m", platform);

    Command::new("rm")
        .arg("-rf")
        .arg("package-lock.json")
        .status()
        .map_err(|_| AppError::CommandFailed("rm -rf package-lock.json".to_string()))?;
    println!("\x1b[32m[rn-run]: package-lock.json deleted\x1b[0m");

    if platform == "ios" {
        Command::new("rm")
            .arg("-rf")
            .arg("ios/Pods")
            .status()
            .map_err(|_| AppError::CommandFailed("rm -rf ios/Pods".to_string()))?;
        println!("\x1b[32m[rn-run]: ios/Pods deleted\x1b[0m");

        Command::new("rm")
            .arg("-rf")
            .arg("ios/build")
            .status()
            .map_err(|_| AppError::CommandFailed("rm -rf ios/build".to_string()))?;
        println!("\x1b[32m[rn-run]: ios/build deleted\x1b[0m");

        Command::new("rm")
            .arg("-rf")
            .arg("ios/Podfile.lock")
            .status()
            .map_err(|_| AppError::CommandFailed("rm -rf ios/Podfile.lock".to_string()))?;
        println!("\x1b[32m[rn-run]: ios/Podfile.lock deleted\x1b[0m");
    } else if platform == "android" {
        Command::new("rm")
            .arg("-rf")
            .arg("android/build")
            .status()
            .map_err(|_| AppError::CommandFailed("rm -rf android/build".to_string()))?;
        println!("\x1b[32m[rn-run]: android/build deleted\x1b[0m");

        Command::new("rm")
            .arg("-rf")
            .arg("android/app/build")
            .status()
            .map_err(|_| AppError::CommandFailed("rm -rf android/app/build".to_string()))?;
        println!("\x1b[32m[rn-run]: android/app/build deleted\x1b[0m");

        Command::new("rm")
            .arg("-rf")
            .arg("android/.gradle")
            .status()
            .map_err(|_| AppError::CommandFailed("rm -rf android/.gradle".to_string()))?;
        println!("\x1b[32m[rn-run]: android/.gradle deleted\x1b[0m");
    }

    Ok(())
}

pub fn watch_directory_logged(watch_dir: &str, log: Option<&LogWriter>) -> Result<()> {
    let msg = format!("Watching directory: {}", watch_dir);
    if let Some(log) = log {
        log.log(&msg);
    } else {
        println!("{}", msg);
    }

    let output1 = Command::new("watchman")
        .arg("watch-del")
        .arg(watch_dir)
        .output()
        .map_err(|_| AppError::CommandFailed("watchman watch-del".to_string()))?;

    if let Some(log) = log {
        log.log_command_output(&output1);
    } else {
        print!("{}", String::from_utf8_lossy(&output1.stdout));
    }

    let output2 = Command::new("watchman")
        .arg("watch-project")
        .arg(watch_dir)
        .output()
        .map_err(|_| AppError::CommandFailed("watchman watch-project".to_string()))?;

    if let Some(log) = log {
        log.log_command_output(&output2);
    } else {
        print!("{}", String::from_utf8_lossy(&output2.stdout));
    }

    Ok(())
}

pub fn watch_directory(watch_dir: &str) -> Result<()> {
    watch_directory_logged(watch_dir, None)
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

pub fn launch_sim(react_native_version: &str, args: &Args, log_writer: &LogWriter) -> Result<String> {
    let log_path = &log_writer.path;

    let yarn_ios = "yarn react-native run-ios".to_string();
    let yarn_android = "yarn react-native run-android --active-arch-only".to_string();
    let npx_ios = format!("npx react-native run-ios --simulator=\"{}\"", args.simulator.as_ref().unwrap_or(&"iPhone 15".to_string()));
    let npx_android = "npx react-native run-android --active-arch-only".to_string();

    let base_command = if args.ios && is_version_greater_or_equal(react_native_version, "0.74") {
        npx_ios
    } else if args.ios && react_native_version.starts_with("0.69") {
        yarn_ios
    } else if args.android && is_version_greater_or_equal(react_native_version, "0.74") {
        npx_android
    } else if args.android && react_native_version.starts_with("0.69") {
        yarn_android
    } else {
        "echo \"No platform specified, use --help for more info\"".to_string()
    };

    let current_dir = env::current_dir()
        .map_err(|_| AppError::CurrentDir)?
        .to_str()
        .ok_or(AppError::CurrentDir)?
        .to_owned();

    // Check for prebuild script in package.json and prepend if it exists
    let package_json_path = PathBuf::from(&current_dir).join("package.json");
    let build_command = if has_prebuild_script(&package_json_path) {
        log_writer.log_green("[rn-run]: Found prebuild script, running npm run prebuild first");
        format!("npm run prebuild && {}", base_command)
    } else {
        base_command
    };

    log_writer.log_green(&format!("[rn-run]: Logging to {}", log_path));

    // Use 'script' to capture output while preserving full TTY behavior (colors, animations, spinners)
    // script -q = quiet, -a = append to existing log file
    let escaped_build_cmd = build_command.replace("'", "'\"'\"'");
    let script_command = format!(
        "script -q -a '{}' bash -c '{}'",
        log_path.replace("'", "'\"'\"'"),
        escaped_build_cmd
    );

    let osascript_command = format!(
        "tell application \"Terminal\" to do script \"cd {}; {}\"",
        current_dir,
        script_command.replace("\"", "\\\"")
    );

    Command::new("osascript")
        .arg("-e")
        .arg(&osascript_command)
        .status()
        .map_err(|_| AppError::CommandFailed("launch simulator".to_string()))?;

    Ok(log_path.clone())
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

#[derive(Deserialize)]
struct CratesIoResponse {
    #[serde(rename = "crate")]
    krate: CrateInfo,
}

#[derive(Deserialize)]
struct CrateInfo {
    max_version: String,
}

pub struct UpdateResult {
    pub current_version: String,
    pub latest_version: String,
    pub updated: bool,
    pub message: String,
}

pub fn check_and_update() -> Result<UpdateResult> {
    let current_version = env!("CARGO_PKG_VERSION");

    // Fetch latest version from crates.io
    let output = Command::new("curl")
        .args(["-s", "https://crates.io/api/v1/crates/rn-run"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .map_err(|_| AppError::CommandFailed("curl crates.io".to_string()))?;

    if !output.status.success() {
        return Err(AppError::CommandFailed("Failed to fetch version info from crates.io".to_string()));
    }

    let response: CratesIoResponse = serde_json::from_slice(&output.stdout)
        .map_err(|_| AppError::CommandFailed("Failed to parse crates.io response".to_string()))?;

    let latest_version = &response.krate.max_version;

    // Compare versions
    if is_version_greater_or_equal(current_version, latest_version) {
        return Ok(UpdateResult {
            current_version: current_version.to_string(),
            latest_version: latest_version.clone(),
            updated: false,
            message: format!("Already up to date (v{})", current_version),
        });
    }

    // Update to latest version
    println!("\x1b[32m[rn-run]: Updating from v{} to v{}...\x1b[0m", current_version, latest_version);

    let status = Command::new("cargo")
        .args(["install", "rn-run", "--force"])
        .status()
        .map_err(|_| AppError::CommandFailed("cargo install rn-run".to_string()))?;

    if !status.success() {
        return Err(AppError::CommandFailed("cargo install rn-run failed".to_string()));
    }

    Ok(UpdateResult {
        current_version: current_version.to_string(),
        latest_version: latest_version.clone(),
        updated: true,
        message: format!("Updated from v{} to v{}", current_version, latest_version),
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// LOGGING FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn get_log_dir() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(LOG_DIR)
}

pub fn ensure_log_dir() -> Result<PathBuf> {
    let log_dir = get_log_dir();
    if !log_dir.exists() {
        fs::create_dir_all(&log_dir)
            .map_err(|_| AppError::CommandFailed("Failed to create log directory".to_string()))?;
    }
    Ok(log_dir)
}

pub fn create_log_path(platform: &str) -> Result<String> {
    let log_dir = ensure_log_dir()?;
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let log_file = log_dir.join(format!("rn-run-{}-{}.log", platform, timestamp));
    Ok(log_file.to_string_lossy().to_string())
}

pub fn rotate_logs() -> Result<()> {
    let log_dir = get_log_dir();
    if !log_dir.exists() {
        return Ok(());
    }

    let mut logs: Vec<_> = fs::read_dir(&log_dir)
        .map_err(|_| AppError::CommandFailed("Failed to read log directory".to_string()))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension().map(|e| e == "log").unwrap_or(false)
        })
        .collect();

    // Sort by modification time (newest first)
    logs.sort_by(|a, b| {
        let a_time = a.metadata().and_then(|m| m.modified()).ok();
        let b_time = b.metadata().and_then(|m| m.modified()).ok();
        b_time.cmp(&a_time)
    });

    // Delete logs beyond MAX_LOGS
    for log in logs.iter().skip(MAX_LOGS) {
        let _ = fs::remove_file(log.path());
    }

    Ok(())
}

#[derive(Clone)]
pub struct LogEntry {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub modified: String,
}

pub fn list_logs() -> Result<Vec<LogEntry>> {
    let log_dir = get_log_dir();
    if !log_dir.exists() {
        return Ok(Vec::new());
    }

    let mut logs: Vec<_> = fs::read_dir(&log_dir)
        .map_err(|_| AppError::CommandFailed("Failed to read log directory".to_string()))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension().map(|e| e == "log").unwrap_or(false)
        })
        .collect();

    // Sort by modification time (newest first)
    logs.sort_by(|a, b| {
        let a_time = a.metadata().and_then(|m| m.modified()).ok();
        let b_time = b.metadata().and_then(|m| m.modified()).ok();
        b_time.cmp(&a_time)
    });

    let entries: Vec<LogEntry> = logs
        .iter()
        .filter_map(|entry| {
            let path = entry.path();
            let metadata = entry.metadata().ok()?;
            let modified = metadata.modified().ok()?;
            let datetime: chrono::DateTime<Local> = modified.into();

            Some(LogEntry {
                path: path.to_string_lossy().to_string(),
                name: path.file_name()?.to_string_lossy().to_string(),
                size: metadata.len(),
                modified: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            })
        })
        .collect();

    Ok(entries)
}

pub fn get_latest_log() -> Result<Option<LogEntry>> {
    let logs = list_logs()?;
    Ok(logs.into_iter().next())
}

/// Strip ANSI escape codes from text
fn strip_ansi_codes(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip escape sequence
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                // Skip until we hit a letter (end of escape sequence)
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else if c == '\r' {
            // Skip carriage returns (used for progress animations)
            continue;
        } else {
            result.push(c);
        }
    }

    result
}

/// Clean up log content by:
/// 1. Stripping ANSI escape codes (colors, cursor movement)
/// 2. Deduplicating progress spinner lines (- Building..., etc.)
pub fn clean_log_content(content: &str) -> String {
    use std::collections::HashSet;

    // First strip ANSI codes
    let clean = strip_ansi_codes(content);

    let mut seen_progress: HashSet<String> = HashSet::new();
    let mut result = Vec::new();

    for line in clean.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Check if this is a progress spinner line (starts with "- " and ends with dots)
        if trimmed.starts_with("- ") && trimmed.ends_with('.') {
            // Extract the base message without trailing dots
            let base = trimmed.trim_end_matches('.');
            if !seen_progress.contains(base) {
                seen_progress.insert(base.to_string());
                result.push(format!("{}...", base));
            }
            // Skip duplicate progress lines
        } else {
            result.push(trimmed.to_string());
        }
    }

    result.join("\n")
}

// ═══════════════════════════════════════════════════════════════════════════════
// LOG WRITER - writes to both console and log file
// ═══════════════════════════════════════════════════════════════════════════════

use std::io::Write;
use std::fs::OpenOptions;

pub struct LogWriter {
    pub path: String,
}

impl LogWriter {
    pub fn new(platform: &str) -> Result<Self> {
        rotate_logs()?;
        let path = create_log_path(platform)?;

        // Create the log file with a header
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .map_err(|_| AppError::CommandFailed("Failed to create log file".to_string()))?;

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(file, "=== rn-run {} build log ===", platform).ok();
        writeln!(file, "Started: {}", timestamp).ok();
        writeln!(file, "").ok();

        Ok(LogWriter { path })
    }

    pub fn log(&self, message: &str) {
        // Print to console
        println!("{}", message);

        // Append to log file
        if let Ok(mut file) = OpenOptions::new().append(true).open(&self.path) {
            writeln!(file, "{}", message).ok();
        }
    }

    pub fn log_green(&self, message: &str) {
        // Print to console with green color
        println!("\x1b[32m{}\x1b[0m", message);

        // Append to log file (without color codes)
        if let Ok(mut file) = OpenOptions::new().append(true).open(&self.path) {
            writeln!(file, "{}", message).ok();
        }
    }

    pub fn log_command_output(&self, output: &std::process::Output) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stdout.is_empty() {
            print!("{}", stdout);
            if let Ok(mut file) = OpenOptions::new().append(true).open(&self.path) {
                write!(file, "{}", stdout).ok();
            }
        }
        if !stderr.is_empty() {
            eprint!("{}", stderr);
            if let Ok(mut file) = OpenOptions::new().append(true).open(&self.path) {
                write!(file, "{}", stderr).ok();
            }
        }
    }
}