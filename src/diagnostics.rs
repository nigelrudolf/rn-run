use std::process::Command;
use crate::output::{EnvCheck, EnvCheckResult, SimulatorListResult, Simulator, EmulatorListResult};
use serde::Deserialize;

/// Check all development environment dependencies.
/// Returns structured data about each tool's availability and version.
pub fn check_environment() -> EnvCheckResult {
    let mut checks = Vec::new();
    let mut has_errors = false;
    let mut has_warnings = false;

    // Node.js
    checks.push(check_node());

    // npm
    checks.push(check_npm());

    // Yarn
    checks.push(check_yarn());

    // Watchman
    checks.push(check_watchman());

    // Xcode (iOS)
    checks.push(check_xcode());

    // CocoaPods (iOS)
    checks.push(check_cocoapods());

    // Ruby (iOS - needed for CocoaPods)
    checks.push(check_ruby());

    // Android SDK
    checks.push(check_android_sdk());

    // Java (Android)
    checks.push(check_java());

    // Calculate overall status
    for check in &checks {
        if !check.ok {
            if check.required_for.contains(&"ios".to_string()) ||
               check.required_for.contains(&"android".to_string()) {
                has_errors = true;
            } else {
                has_warnings = true;
            }
        }
    }

    let overall_status = if has_errors {
        "errors"
    } else if has_warnings {
        "warnings"
    } else {
        "ok"
    }.to_string();

    let summary = if has_errors {
        "Some required tools are missing. See 'fix' field for each failed check.".to_string()
    } else if has_warnings {
        "All required tools present. Some optional tools missing.".to_string()
    } else {
        "All development tools are properly installed.".to_string()
    };

    EnvCheckResult {
        overall_status,
        checks,
        summary,
    }
}

fn check_node() -> EnvCheck {
    match get_command_version("node", &["--version"]) {
        Some(version) => EnvCheck {
            name: "node".to_string(),
            ok: true,
            version: Some(version),
            error: None,
            fix: None,
            required_for: vec!["ios".to_string(), "android".to_string()],
        },
        None => EnvCheck {
            name: "node".to_string(),
            ok: false,
            version: None,
            error: Some("Node.js not found".to_string()),
            fix: Some("Install Node.js: brew install node OR https://nodejs.org".to_string()),
            required_for: vec!["ios".to_string(), "android".to_string()],
        },
    }
}

fn check_npm() -> EnvCheck {
    match get_command_version("npm", &["--version"]) {
        Some(version) => EnvCheck {
            name: "npm".to_string(),
            ok: true,
            version: Some(version),
            error: None,
            fix: None,
            required_for: vec!["ios".to_string(), "android".to_string()],
        },
        None => EnvCheck {
            name: "npm".to_string(),
            ok: false,
            version: None,
            error: Some("npm not found".to_string()),
            fix: Some("npm comes with Node.js. Reinstall Node.js.".to_string()),
            required_for: vec!["ios".to_string(), "android".to_string()],
        },
    }
}

fn check_yarn() -> EnvCheck {
    match get_command_version("yarn", &["--version"]) {
        Some(version) => EnvCheck {
            name: "yarn".to_string(),
            ok: true,
            version: Some(version),
            error: None,
            fix: None,
            required_for: vec![], // Optional, older RN versions
        },
        None => EnvCheck {
            name: "yarn".to_string(),
            ok: false,
            version: None,
            error: Some("yarn not found (optional, needed for RN < 0.74)".to_string()),
            fix: Some("Install yarn: npm install -g yarn".to_string()),
            required_for: vec![],
        },
    }
}

fn check_watchman() -> EnvCheck {
    match get_command_version("watchman", &["--version"]) {
        Some(version) => EnvCheck {
            name: "watchman".to_string(),
            ok: true,
            version: Some(version),
            error: None,
            fix: None,
            required_for: vec!["ios".to_string(), "android".to_string()],
        },
        None => EnvCheck {
            name: "watchman".to_string(),
            ok: false,
            version: None,
            error: Some("watchman not found".to_string()),
            fix: Some("Install watchman: brew install watchman".to_string()),
            required_for: vec!["ios".to_string(), "android".to_string()],
        },
    }
}

fn check_xcode() -> EnvCheck {
    // Check xcodebuild version
    match get_command_version("xcodebuild", &["-version"]) {
        Some(version) => {
            // Also check if command line tools are accepted
            let license_check = Command::new("xcodebuild")
                .arg("-checkFirstLaunchStatus")
                .output();

            let license_ok = license_check.map(|o| o.status.success()).unwrap_or(false);

            if license_ok {
                EnvCheck {
                    name: "xcode".to_string(),
                    ok: true,
                    version: Some(version),
                    error: None,
                    fix: None,
                    required_for: vec!["ios".to_string()],
                }
            } else {
                EnvCheck {
                    name: "xcode".to_string(),
                    ok: false,
                    version: Some(version),
                    error: Some("Xcode license not accepted or first launch not complete".to_string()),
                    fix: Some("Run: sudo xcodebuild -license accept".to_string()),
                    required_for: vec!["ios".to_string()],
                }
            }
        },
        None => EnvCheck {
            name: "xcode".to_string(),
            ok: false,
            version: None,
            error: Some("Xcode not found".to_string()),
            fix: Some("Install Xcode from the Mac App Store, then run: xcode-select --install".to_string()),
            required_for: vec!["ios".to_string()],
        },
    }
}

fn check_cocoapods() -> EnvCheck {
    match get_command_version("pod", &["--version"]) {
        Some(version) => EnvCheck {
            name: "cocoapods".to_string(),
            ok: true,
            version: Some(version),
            error: None,
            fix: None,
            required_for: vec!["ios".to_string()],
        },
        None => EnvCheck {
            name: "cocoapods".to_string(),
            ok: false,
            version: None,
            error: Some("CocoaPods not found".to_string()),
            fix: Some("Install CocoaPods: sudo gem install cocoapods OR brew install cocoapods".to_string()),
            required_for: vec!["ios".to_string()],
        },
    }
}

fn check_ruby() -> EnvCheck {
    match get_command_version("ruby", &["--version"]) {
        Some(version) => EnvCheck {
            name: "ruby".to_string(),
            ok: true,
            version: Some(version),
            error: None,
            fix: None,
            required_for: vec!["ios".to_string()],
        },
        None => EnvCheck {
            name: "ruby".to_string(),
            ok: false,
            version: None,
            error: Some("Ruby not found".to_string()),
            fix: Some("Ruby should be pre-installed on macOS. Try: brew install ruby".to_string()),
            required_for: vec!["ios".to_string()],
        },
    }
}

fn check_android_sdk() -> EnvCheck {
    // Check ANDROID_HOME or ANDROID_SDK_ROOT environment variable
    let android_home = std::env::var("ANDROID_HOME")
        .or_else(|_| std::env::var("ANDROID_SDK_ROOT"));

    match android_home {
        Ok(path) => {
            // Verify the path exists
            if std::path::Path::new(&path).exists() {
                EnvCheck {
                    name: "android_sdk".to_string(),
                    ok: true,
                    version: Some(format!("ANDROID_HOME={}", path)),
                    error: None,
                    fix: None,
                    required_for: vec!["android".to_string()],
                }
            } else {
                EnvCheck {
                    name: "android_sdk".to_string(),
                    ok: false,
                    version: None,
                    error: Some(format!("ANDROID_HOME points to non-existent path: {}", path)),
                    fix: Some("Install Android Studio and set ANDROID_HOME to the SDK location".to_string()),
                    required_for: vec!["android".to_string()],
                }
            }
        },
        Err(_) => EnvCheck {
            name: "android_sdk".to_string(),
            ok: false,
            version: None,
            error: Some("ANDROID_HOME not set".to_string()),
            fix: Some("Install Android Studio, then add to ~/.zshrc: export ANDROID_HOME=$HOME/Library/Android/sdk".to_string()),
            required_for: vec!["android".to_string()],
        },
    }
}

fn check_java() -> EnvCheck {
    match get_command_version("java", &["-version"]) {
        Some(version) => EnvCheck {
            name: "java".to_string(),
            ok: true,
            version: Some(version),
            error: None,
            fix: None,
            required_for: vec!["android".to_string()],
        },
        None => EnvCheck {
            name: "java".to_string(),
            ok: false,
            version: None,
            error: Some("Java not found".to_string()),
            fix: Some("Install Java: brew install openjdk@17".to_string()),
            required_for: vec!["android".to_string()],
        },
    }
}

fn get_command_version(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| {
            // Try stdout first, then stderr (java -version outputs to stderr)
            let out = String::from_utf8_lossy(&output.stdout);
            if out.trim().is_empty() {
                String::from_utf8_lossy(&output.stderr).trim().lines().next().unwrap_or("").to_string()
            } else {
                out.trim().lines().next().unwrap_or("").to_string()
            }
        })
}

/// List available iOS simulators using xcrun simctl.
pub fn list_simulators() -> SimulatorListResult {
    let output = Command::new("xcrun")
        .args(["simctl", "list", "devices", "--json"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let json_str = String::from_utf8_lossy(&out.stdout);
            parse_simulators(&json_str)
        },
        _ => SimulatorListResult { simulators: vec![] },
    }
}

#[derive(Deserialize)]
struct SimctlOutput {
    devices: std::collections::HashMap<String, Vec<SimctlDevice>>,
}

#[derive(Deserialize)]
struct SimctlDevice {
    name: String,
    udid: String,
    state: String,
    #[serde(rename = "isAvailable")]
    is_available: Option<bool>,
}

fn parse_simulators(json_str: &str) -> SimulatorListResult {
    let parsed: Result<SimctlOutput, _> = serde_json::from_str(json_str);

    match parsed {
        Ok(simctl) => {
            let mut simulators = Vec::new();
            for (runtime, devices) in simctl.devices {
                // Only include iOS simulators
                if runtime.contains("iOS") {
                    for device in devices {
                        if device.is_available.unwrap_or(true) {
                            simulators.push(Simulator {
                                name: device.name,
                                udid: device.udid,
                                state: device.state,
                                runtime: runtime.clone(),
                            });
                        }
                    }
                }
            }
            SimulatorListResult { simulators }
        },
        Err(_) => SimulatorListResult { simulators: vec![] },
    }
}

/// List available Android emulators using emulator command.
pub fn list_emulators() -> EmulatorListResult {
    let output = Command::new("emulator")
        .arg("-list-avds")
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let emulators: Vec<String> = String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|s| s.to_string())
                .collect();
            EmulatorListResult { emulators }
        },
        _ => EmulatorListResult { emulators: vec![] },
    }
}
