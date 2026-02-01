use std::process::Command;
use crate::output::{EnvCheck, EnvCheckResult, SimulatorListResult, Simulator, EmulatorListResult};
use serde::Deserialize;

/// Check all development environment dependencies.
/// Returns structured data about each tool's availability and version.
pub fn check_environment() -> EnvCheckResult {
    let mut checks = Vec::new();
    let mut has_errors = false;
    let mut has_warnings = false;

    // macOS version
    checks.push(check_macos());

    // Command Line Tools
    checks.push(check_clt());

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

    // Bundler (iOS - for Gemfile management)
    checks.push(check_bundler());

    // Swift (iOS)
    checks.push(check_swift());

    // Android SDK
    checks.push(check_android_sdk());

    // Java (Android)
    checks.push(check_java());

    // Gradle (Android)
    checks.push(check_gradle());

    // Android Gradle Plugin (Android)
    checks.push(check_android_gradle_plugin());

    // Kotlin (Android)
    checks.push(check_kotlin());

    // Android NDK (Android)
    checks.push(check_ndk());

    // Android Build Tools (Android)
    checks.push(check_build_tools());

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

fn check_macos() -> EnvCheck {
    match get_command_version("sw_vers", &["-productVersion"]) {
        Some(version) => EnvCheck {
            name: "macos".to_string(),
            ok: true,
            version: Some(format!("macOS {}", version)),
            error: None,
            fix: None,
            required_for: vec!["ios".to_string(), "android".to_string()],
        },
        None => EnvCheck {
            name: "macos".to_string(),
            ok: false,
            version: None,
            error: Some("Could not determine macOS version".to_string()),
            fix: None,
            required_for: vec!["ios".to_string(), "android".to_string()],
        },
    }
}

fn check_clt() -> EnvCheck {
    // Check Command Line Tools version
    let output = Command::new("pkgutil")
        .args(["--pkg-info=com.apple.pkg.CLTools_Executables"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let version = stdout.lines()
                .find(|line| line.starts_with("version:"))
                .map(|line| line.replace("version:", "").trim().to_string())
                .unwrap_or_else(|| "installed".to_string());

            EnvCheck {
                name: "clt".to_string(),
                ok: true,
                version: Some(format!("Command Line Tools {}", version)),
                error: None,
                fix: None,
                required_for: vec!["ios".to_string()],
            }
        },
        _ => EnvCheck {
            name: "clt".to_string(),
            ok: false,
            version: None,
            error: Some("Command Line Tools not installed".to_string()),
            fix: Some("Install Command Line Tools: xcode-select --install".to_string()),
            required_for: vec!["ios".to_string()],
        },
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

fn check_bundler() -> EnvCheck {
    match get_command_version("bundler", &["--version"]) {
        Some(version) => EnvCheck {
            name: "bundler".to_string(),
            ok: true,
            version: Some(version),
            error: None,
            fix: None,
            required_for: vec![], // Optional, but useful for iOS
        },
        None => EnvCheck {
            name: "bundler".to_string(),
            ok: false,
            version: None,
            error: Some("Bundler not found (optional, for Gemfile management)".to_string()),
            fix: Some("Install Bundler: gem install bundler".to_string()),
            required_for: vec![],
        },
    }
}

fn check_swift() -> EnvCheck {
    match get_command_version("swift", &["--version"]) {
        Some(version) => {
            // Extract just the Swift version from output like "swift-driver version: 1.87.1 Apple Swift version 5.9.2..."
            let swift_version = version.lines()
                .find(|line| line.contains("Swift version"))
                .map(|line| {
                    line.split("Swift version")
                        .nth(1)
                        .map(|s| format!("Swift{}", s.split('(').next().unwrap_or("").trim()))
                        .unwrap_or_else(|| version.clone())
                })
                .unwrap_or(version);

            EnvCheck {
                name: "swift".to_string(),
                ok: true,
                version: Some(swift_version),
                error: None,
                fix: None,
                required_for: vec![], // Optional, for Swift-based native modules
            }
        },
        None => EnvCheck {
            name: "swift".to_string(),
            ok: false,
            version: None,
            error: Some("Swift not found (optional, for Swift-based modules)".to_string()),
            fix: Some("Swift comes with Xcode. Install Xcode from the App Store.".to_string()),
            required_for: vec![],
        },
    }
}

fn check_gradle() -> EnvCheck {
    // Check if we're in a React Native project with android directory
    let has_android_dir = std::path::Path::new("android").exists();

    // Try to get Gradle version from the project's gradlew wrapper first
    let gradlew_output = Command::new("./android/gradlew")
        .args(["--version"])
        .output();

    if let Ok(out) = gradlew_output {
        if out.status.success() {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let version = stdout.lines()
                .find(|line| line.starts_with("Gradle "))
                .map(|line| line.to_string())
                .unwrap_or_else(|| "installed".to_string());

            return EnvCheck {
                name: "gradle".to_string(),
                ok: true,
                version: Some(version),
                error: None,
                fix: None,
                required_for: vec!["android".to_string()],
            };
        }
    }

    // Fall back to system gradle
    match get_command_version("gradle", &["--version"]) {
        Some(version) => {
            let version_line = version.lines()
                .find(|line| line.starts_with("Gradle "))
                .unwrap_or(&version)
                .to_string();

            EnvCheck {
                name: "gradle".to_string(),
                ok: true,
                version: Some(version_line),
                error: None,
                fix: None,
                required_for: vec!["android".to_string()],
            }
        },
        None => EnvCheck {
            name: "gradle".to_string(),
            ok: false,
            version: None,
            error: Some("Gradle not found (run from RN project directory for gradlew)".to_string()),
            fix: Some("Gradle is bundled with Android projects. Run from your RN project directory.".to_string()),
            // Only mark as required if we're in a project with android directory
            required_for: if has_android_dir { vec!["android".to_string()] } else { vec![] },
        },
    }
}

fn check_android_gradle_plugin() -> EnvCheck {
    // Check if we're in a React Native project with android directory
    let has_android_dir = std::path::Path::new("android").exists();

    // Try multiple sources for AGP version (newer RN projects use different locations)
    let version = find_agp_version();

    match version {
        Some(v) => EnvCheck {
            name: "agp".to_string(),
            ok: true,
            version: Some(format!("Android Gradle Plugin {}", v)),
            error: None,
            fix: None,
            required_for: vec!["android".to_string()],
        },
        None => {
            if has_android_dir {
                EnvCheck {
                    name: "agp".to_string(),
                    ok: true,
                    version: Some("Could not parse AGP version".to_string()),
                    error: None,
                    fix: None,
                    required_for: vec!["android".to_string()],
                }
            } else {
                EnvCheck {
                    name: "agp".to_string(),
                    ok: false,
                    version: None,
                    error: Some("android/ not found (run from RN project directory)".to_string()),
                    fix: Some("Run this command from your React Native project directory.".to_string()),
                    required_for: vec![],
                }
            }
        }
    }
}

fn find_agp_version() -> Option<String> {
    // 1. Check gradle/libs.versions.toml (version catalog - newest approach)
    if let Ok(content) = std::fs::read_to_string("android/gradle/libs.versions.toml") {
        // Look for: agp = "8.5.0" or androidGradlePlugin = "8.5.0"
        for line in content.lines() {
            let trimmed = line.trim();
            if (trimmed.starts_with("agp") || trimmed.starts_with("androidGradlePlugin"))
                && trimmed.contains('=') {
                if let Some(version) = extract_quoted_version(trimmed) {
                    return Some(version);
                }
            }
        }
    }

    // 2. Check android/settings.gradle for plugin DSL
    if let Ok(content) = std::fs::read_to_string("android/settings.gradle") {
        if let Some(v) = parse_agp_from_gradle(&content) {
            return Some(v);
        }
    }

    // 3. Check android/settings.gradle.kts
    if let Ok(content) = std::fs::read_to_string("android/settings.gradle.kts") {
        if let Some(v) = parse_agp_from_gradle(&content) {
            return Some(v);
        }
    }

    // 4. Check android/build.gradle (classic approach)
    if let Ok(content) = std::fs::read_to_string("android/build.gradle") {
        if let Some(v) = parse_agp_from_gradle(&content) {
            return Some(v);
        }
    }

    // 5. Check android/build.gradle.kts
    if let Ok(content) = std::fs::read_to_string("android/build.gradle.kts") {
        if let Some(v) = parse_agp_from_gradle(&content) {
            return Some(v);
        }
    }

    // 6. Check React Native gradle plugin's version catalog (RN 0.74+)
    if let Ok(content) = std::fs::read_to_string("node_modules/@react-native/gradle-plugin/gradle/libs.versions.toml") {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("agp") && trimmed.contains('=') {
                if let Some(version) = extract_quoted_version(trimmed) {
                    return Some(version);
                }
            }
        }
    }

    None
}

fn parse_agp_from_gradle(content: &str) -> Option<String> {
    for line in content.lines() {
        // classpath("com.android.tools.build:gradle:8.0.0")
        // classpath 'com.android.tools.build:gradle:8.0.0'
        if line.contains("com.android.tools.build:gradle:") {
            return line.split("gradle:")
                .nth(1)
                .and_then(|s| s.split(|c| c == '"' || c == '\'' || c == ')').next())
                .map(|s| s.to_string());
        }

        // id("com.android.application") version "8.0.0"
        // id 'com.android.application' version '8.0.0'
        if line.contains("com.android.application") && line.contains("version") {
            return extract_version_after_keyword(line, "version");
        }
    }
    None
}

fn extract_quoted_version(line: &str) -> Option<String> {
    // Extract version from: key = "1.2.3" or key = '1.2.3'
    line.split(|c| c == '"' || c == '\'')
        .find(|s| s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
        .map(|s| s.to_string())
}

fn extract_version_after_keyword(line: &str, keyword: &str) -> Option<String> {
    line.split(keyword)
        .nth(1)
        .and_then(|s| {
            s.split(|c| c == '"' || c == '\'')
                .find(|s| s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
        })
        .map(|s| s.to_string())
}

fn check_kotlin() -> EnvCheck {
    // Check if we're in a React Native project with android directory
    let has_android_dir = std::path::Path::new("android").exists();

    // Try multiple sources for Kotlin version
    let version = find_kotlin_version();

    match version {
        Some(v) => EnvCheck {
            name: "kotlin".to_string(),
            ok: true,
            version: Some(format!("Kotlin {}", v)),
            error: None,
            fix: None,
            required_for: vec!["android".to_string()],
        },
        None => {
            if has_android_dir {
                EnvCheck {
                    name: "kotlin".to_string(),
                    ok: true,
                    version: Some("Could not parse Kotlin version".to_string()),
                    error: None,
                    fix: None,
                    required_for: vec!["android".to_string()],
                }
            } else {
                EnvCheck {
                    name: "kotlin".to_string(),
                    ok: false,
                    version: None,
                    error: Some("android/ not found (run from RN project directory)".to_string()),
                    fix: Some("Run this command from your React Native project directory.".to_string()),
                    required_for: vec![],
                }
            }
        }
    }
}

fn find_kotlin_version() -> Option<String> {
    // 1. Check gradle/libs.versions.toml (version catalog)
    if let Ok(content) = std::fs::read_to_string("android/gradle/libs.versions.toml") {
        for line in content.lines() {
            let trimmed = line.trim();
            // Look for: kotlin = "1.9.24" or kotlinVersion = "1.9.24"
            if (trimmed.starts_with("kotlin") && !trimmed.contains("kotlinCompilerExtensionVersion"))
                && trimmed.contains('=') {
                if let Some(version) = extract_quoted_version(trimmed) {
                    return Some(version);
                }
            }
        }
    }

    // 2. Check android/settings.gradle
    if let Ok(content) = std::fs::read_to_string("android/settings.gradle") {
        if let Some(v) = parse_kotlin_from_gradle(&content) {
            return Some(v);
        }
    }

    // 3. Check android/settings.gradle.kts
    if let Ok(content) = std::fs::read_to_string("android/settings.gradle.kts") {
        if let Some(v) = parse_kotlin_from_gradle(&content) {
            return Some(v);
        }
    }

    // 4. Check android/build.gradle
    if let Ok(content) = std::fs::read_to_string("android/build.gradle") {
        if let Some(v) = parse_kotlin_from_gradle(&content) {
            return Some(v);
        }
    }

    // 5. Check android/build.gradle.kts
    if let Ok(content) = std::fs::read_to_string("android/build.gradle.kts") {
        if let Some(v) = parse_kotlin_from_gradle(&content) {
            return Some(v);
        }
    }

    None
}

fn parse_kotlin_from_gradle(content: &str) -> Option<String> {
    for line in content.lines() {
        // kotlinVersion = "1.9.0" or ext.kotlinVersion = '1.9.0'
        if line.contains("kotlinVersion") && line.contains('=') {
            return extract_quoted_version(line);
        }

        // id("org.jetbrains.kotlin.android") version "1.9.0"
        // id 'org.jetbrains.kotlin.android' version '1.9.0'
        if line.contains("org.jetbrains.kotlin") && line.contains("version") {
            return extract_version_after_keyword(line, "version");
        }

        // classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:1.9.0")
        if line.contains("kotlin-gradle-plugin:") {
            return line.split("kotlin-gradle-plugin:")
                .nth(1)
                .and_then(|s| s.split(|c| c == '"' || c == '\'' || c == ')').next())
                .map(|s| s.to_string());
        }
    }
    None
}

fn check_ndk() -> EnvCheck {
    let android_home = std::env::var("ANDROID_HOME")
        .or_else(|_| std::env::var("ANDROID_SDK_ROOT"));

    match android_home {
        Ok(sdk_path) => {
            let ndk_path = std::path::Path::new(&sdk_path).join("ndk");

            if ndk_path.exists() {
                // List NDK versions installed
                if let Ok(entries) = std::fs::read_dir(&ndk_path) {
                    let versions: Vec<String> = entries
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().is_dir())
                        .filter_map(|e| e.file_name().into_string().ok())
                        .filter(|name| name.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
                        .collect();

                    if !versions.is_empty() {
                        return EnvCheck {
                            name: "ndk".to_string(),
                            ok: true,
                            version: Some(format!("NDK {}", versions.join(", "))),
                            error: None,
                            fix: None,
                            required_for: vec![], // Optional, for native modules
                        };
                    }
                }
            }

            // Check for legacy ndk-bundle
            let ndk_bundle = std::path::Path::new(&sdk_path).join("ndk-bundle");
            if ndk_bundle.exists() {
                // Try to read source.properties for version
                let props_path = ndk_bundle.join("source.properties");
                if let Ok(content) = std::fs::read_to_string(props_path) {
                    let version = content.lines()
                        .find(|line| line.starts_with("Pkg.Revision"))
                        .and_then(|line| line.split('=').nth(1))
                        .map(|v| v.trim().to_string())
                        .unwrap_or_else(|| "installed".to_string());

                    return EnvCheck {
                        name: "ndk".to_string(),
                        ok: true,
                        version: Some(format!("NDK {}", version)),
                        error: None,
                        fix: None,
                        required_for: vec![],
                    };
                }
            }

            EnvCheck {
                name: "ndk".to_string(),
                ok: false,
                version: None,
                error: Some("NDK not installed (optional, for native modules)".to_string()),
                fix: Some("Install NDK via Android Studio > SDK Manager > SDK Tools > NDK".to_string()),
                required_for: vec![],
            }
        },
        Err(_) => EnvCheck {
            name: "ndk".to_string(),
            ok: false,
            version: None,
            error: Some("ANDROID_HOME not set".to_string()),
            fix: Some("Set ANDROID_HOME environment variable".to_string()),
            required_for: vec![],
        },
    }
}

fn check_build_tools() -> EnvCheck {
    let android_home = std::env::var("ANDROID_HOME")
        .or_else(|_| std::env::var("ANDROID_SDK_ROOT"));

    match android_home {
        Ok(sdk_path) => {
            let build_tools_path = std::path::Path::new(&sdk_path).join("build-tools");

            if build_tools_path.exists() {
                if let Ok(entries) = std::fs::read_dir(&build_tools_path) {
                    let mut versions: Vec<String> = entries
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().is_dir())
                        .filter_map(|e| e.file_name().into_string().ok())
                        .collect();

                    versions.sort();
                    versions.reverse(); // Latest first

                    if !versions.is_empty() {
                        return EnvCheck {
                            name: "build_tools".to_string(),
                            ok: true,
                            version: Some(format!("Build Tools {}", versions.first().unwrap())),
                            error: None,
                            fix: None,
                            required_for: vec!["android".to_string()],
                        };
                    }
                }
            }

            EnvCheck {
                name: "build_tools".to_string(),
                ok: false,
                version: None,
                error: Some("Android Build Tools not installed".to_string()),
                fix: Some("Install via Android Studio > SDK Manager > SDK Tools > Android SDK Build-Tools".to_string()),
                required_for: vec!["android".to_string()],
            }
        },
        Err(_) => EnvCheck {
            name: "build_tools".to_string(),
            ok: false,
            version: None,
            error: Some("ANDROID_HOME not set".to_string()),
            fix: Some("Set ANDROID_HOME environment variable".to_string()),
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
                .filter(|line| {
                    let trimmed = line.trim();
                    !trimmed.is_empty() &&
                    !trimmed.starts_with("INFO") &&
                    !trimmed.starts_with("WARNING") &&
                    !trimmed.starts_with("ERROR") &&
                    !trimmed.contains('|')
                })
                .map(|s| s.to_string())
                .collect();
            EmulatorListResult { emulators }
        },
        _ => EmulatorListResult { emulators: vec![] },
    }
}
