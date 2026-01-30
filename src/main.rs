mod args;
mod utils;
mod ios;
mod android;
mod main_test;
mod error;
mod output;
mod diagnostics;

use std::path::Path;
use std::process::Command;

use clap::Parser;
use args::Args;
use ios::run_ios;
use android::run_android;
use utils::{get_current_directory, get_react_native_version, is_version_greater_or_equal};
use error::{AppError, Result};
use output::{Output, ActionResult, RnVersionResult, ScreenshotResult};
use diagnostics::{check_environment, list_simulators, list_emulators};

fn main() {
    let args = Args::parse();

    if let Err(e) = run(&args) {
        if args.json {
            Output::<()>::error("error", &e.to_string(), get_error_fix(&e).as_deref()).print();
        } else {
            eprintln!("Error: {}", e);
        }
        std::process::exit(1);
    }
}

fn run(args: &Args) -> Result<()> {
    // ═══════════════════════════════════════════════════════════════════════════
    // STANDALONE DIAGNOSTIC COMMANDS (don't require RN project)
    // ═══════════════════════════════════════════════════════════════════════════

    if args.check_env {
        let result = check_environment();
        if args.json {
            Output::success("check-env", result).print();
        } else {
            print_env_check_human(&result);
        }
        return Ok(());
    }

    if args.list_simulators {
        let result = list_simulators();
        if args.json {
            Output::success("list-simulators", result).print();
        } else {
            println!("Available iOS Simulators:");
            for sim in &result.simulators {
                println!("  {} ({}) - {}", sim.name, sim.runtime, sim.state);
            }
            if result.simulators.is_empty() {
                println!("  No simulators found. Open Xcode > Settings > Platforms to download simulators.");
                println!("  Tip: Add a simulator named \"iPhone 15\" as it is the default for rn-run.");
            }
        }
        return Ok(());
    }

    if args.list_emulators {
        let result = list_emulators();
        if args.json {
            Output::success("list-emulators", result).print();
        } else {
            println!("Available Android Emulators:");
            for emu in &result.emulators {
                println!("  {}", emu);
            }
            if result.emulators.is_empty() {
                println!("  No emulators found. Open Android Studio > Device Manager to create one.");
            }
        }
        return Ok(());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // PROCESS MANAGEMENT COMMANDS (don't require RN project)
    // ═══════════════════════════════════════════════════════════════════════════

    if args.kill_metro {
        utils::kill_process()?;
        if args.json {
            Output::success("kill-metro", ActionResult {
                action: "kill-metro".to_string(),
                message: "Metro bundler killed (if running)".to_string(),
            }).print();
        } else {
            println!("\x1b[32m[rn-run]: Metro bundler killed (if running)\x1b[0m");
        }
        return Ok(());
    }

    if args.quit_simulator {
        utils::quit_simulator()?;
        if args.json {
            Output::success("quit-simulator", ActionResult {
                action: "quit-simulator".to_string(),
                message: "iOS Simulator quit".to_string(),
            }).print();
        } else {
            println!("\x1b[32m[rn-run]: iOS Simulator quit\x1b[0m");
        }
        return Ok(());
    }

    if args.screenshot {
        let output_path = args.output.as_deref();
        let (platform, path) = if args.android {
            ("android", utils::take_android_screenshot(output_path)?)
        } else {
            ("ios", utils::take_ios_screenshot(output_path)?)
        };

        if args.json {
            Output::success("screenshot", ScreenshotResult {
                platform: platform.to_string(),
                path: path.clone(),
                message: format!("Screenshot saved to {}", path),
            }).print();
        } else {
            println!("\x1b[32m[rn-run]: Screenshot saved to {}\x1b[0m", path);
        }
        return Ok(());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TARGETED CLEANUP COMMANDS (don't require full RN detection)
    // ═══════════════════════════════════════════════════════════════════════════

    if args.clean_modules {
        Command::new("rm")
            .arg("-rf")
            .arg("node_modules")
            .status()
            .map_err(|_| AppError::CommandFailed("rm -rf node_modules".to_string()))?;

        if args.json {
            Output::success("clean-modules", ActionResult {
                action: "clean-modules".to_string(),
                message: "node_modules deleted. Run npm/yarn install to reinstall.".to_string(),
            }).print();
        } else {
            println!("\x1b[32m[rn-run]: node_modules deleted\x1b[0m");
            println!("Run 'npm install' or 'yarn install' to reinstall dependencies.");
        }
        return Ok(());
    }

    if args.clean_pods {
        Command::new("rm").arg("-rf").arg("ios/Pods").status()
            .map_err(|_| AppError::CommandFailed("rm -rf ios/Pods".to_string()))?;
        Command::new("rm").arg("-rf").arg("ios/Podfile.lock").status()
            .map_err(|_| AppError::CommandFailed("rm -rf ios/Podfile.lock".to_string()))?;
        Command::new("rm").arg("-rf").arg("ios/build").status()
            .map_err(|_| AppError::CommandFailed("rm -rf ios/build".to_string()))?;

        if args.json {
            Output::success("clean-pods", ActionResult {
                action: "clean-pods".to_string(),
                message: "Cleaned ios/Pods, ios/Podfile.lock, ios/build. Run --pod-install to reinstall.".to_string(),
            }).print();
        } else {
            println!("\x1b[32m[rn-run]: ios/Pods deleted\x1b[0m");
            println!("\x1b[32m[rn-run]: ios/Podfile.lock deleted\x1b[0m");
            println!("\x1b[32m[rn-run]: ios/build deleted\x1b[0m");
            println!("Run 'rn-run --pod-install' to reinstall pods.");
        }
        return Ok(());
    }

    if args.clean_gradle {
        Command::new("rm").arg("-rf").arg("android/build").status()
            .map_err(|_| AppError::CommandFailed("rm -rf android/build".to_string()))?;
        Command::new("rm").arg("-rf").arg("android/app/build").status()
            .map_err(|_| AppError::CommandFailed("rm -rf android/app/build".to_string()))?;
        Command::new("rm").arg("-rf").arg("android/.gradle").status()
            .map_err(|_| AppError::CommandFailed("rm -rf android/.gradle".to_string()))?;

        if args.json {
            Output::success("clean-gradle", ActionResult {
                action: "clean-gradle".to_string(),
                message: "Cleaned android/build, android/app/build, android/.gradle".to_string(),
            }).print();
        } else {
            println!("\x1b[32m[rn-run]: android/build deleted\x1b[0m");
            println!("\x1b[32m[rn-run]: android/app/build deleted\x1b[0m");
            println!("\x1b[32m[rn-run]: android/.gradle deleted\x1b[0m");
        }
        return Ok(());
    }

    if args.clean_metro {
        let tmpdir = std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        Command::new("sh")
            .arg("-c")
            .arg(format!("rm -rf {}metro-* {}haste-map-*", tmpdir, tmpdir))
            .status()
            .map_err(|_| AppError::CommandFailed("rm -rf metro cache".to_string()))?;

        if args.json {
            Output::success("clean-metro", ActionResult {
                action: "clean-metro".to_string(),
                message: "Metro cache cleared".to_string(),
            }).print();
        } else {
            println!("\x1b[32m[rn-run]: Metro cache cleared\x1b[0m");
        }
        return Ok(());
    }

    if args.delete_simulators {
        let status = Command::new("xcrun")
            .args(["simctl", "delete", "all"])
            .status()
            .map_err(|_| AppError::CommandFailed("xcrun simctl delete all".to_string()))?;

        if status.success() {
            if args.json {
                Output::success("delete-simulators", ActionResult {
                    action: "delete-simulators".to_string(),
                    message: "All iOS simulators deleted. Recreate them in Xcode > Settings > Platforms.".to_string(),
                }).print();
            } else {
                println!("\x1b[32m[rn-run]: All iOS simulators deleted\x1b[0m");
                println!("Recreate simulators in Xcode > Settings > Platforms.");
            }
        } else {
            return Err(AppError::CommandFailed("xcrun simctl delete all failed".to_string()));
        }
        return Ok(());
    }

    if args.pod_install {
        let status = Command::new("sh")
            .arg("-c")
            .arg("cd ios && pod install")
            .status()
            .map_err(|_| AppError::CommandFailed("pod install".to_string()))?;

        if status.success() {
            if args.json {
                Output::success("pod-install", ActionResult {
                    action: "pod-install".to_string(),
                    message: "pod install completed successfully".to_string(),
                }).print();
            } else {
                println!("\x1b[32m[rn-run]: pod install completed\x1b[0m");
            }
        } else {
            return Err(AppError::CommandFailed("pod install failed".to_string()));
        }
        return Ok(());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // COMMANDS THAT REQUIRE RN PROJECT CONTEXT
    // ═══════════════════════════════════════════════════════════════════════════

    let current_dir = get_current_directory()?;
    let package_json_path = Path::new(&current_dir).join("package.json");

    let react_native_version = match get_react_native_version(&package_json_path)? {
        Some(version) => version,
        None => return Err(AppError::ReactNativeNotFound),
    };

    // RN Version command
    if args.rn_version {
        let package_manager = if is_version_greater_or_equal(&react_native_version, "0.74") {
            "npm"
        } else {
            "yarn"
        };

        let mut notes = Vec::new();
        if is_version_greater_or_equal(&react_native_version, "0.74") {
            notes.push("RN 0.74+ uses npm and npx for commands".to_string());
        } else {
            notes.push("RN < 0.74 uses yarn for commands".to_string());
        }

        if args.json {
            Output::success("rn-version", RnVersionResult {
                version: react_native_version,
                package_manager: package_manager.to_string(),
                notes,
            }).print();
        } else {
            println!("React Native version: {}", react_native_version);
            println!("Recommended package manager: {}", package_manager);
        }
        return Ok(());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // MAIN RUN COMMANDS
    // ═══════════════════════════════════════════════════════════════════════════

    match args {
        Args { ios: true, android: false, .. } => run_ios(args, &current_dir, &react_native_version),
        Args { android: true, ios: false, .. } => run_android(args, &current_dir, &react_native_version),
        Args { ios: true, android: true, .. } => {
            if args.json {
                Output::<()>::error(
                    "error",
                    "Cannot run both iOS and Android simultaneously",
                    Some("Use either --ios or --android, not both")
                ).print();
            } else {
                eprintln!("Cannot run both iOS and Android simultaneously");
            }
            std::process::exit(1);
        }
        _ => {
            if args.json {
                Output::success("help", ActionResult {
                    action: "help".to_string(),
                    message: "No command specified. Use --help for available options.".to_string(),
                }).print();
            } else {
                println!("No platform specified, use --help for more info");
            }
            Ok(())
        }
    }
}

fn print_env_check_human(result: &output::EnvCheckResult) {
    println!("Environment Check: {}\n", result.overall_status.to_uppercase());

    for check in &result.checks {
        let status = if check.ok { "\x1b[32m✓\x1b[0m" } else { "\x1b[31m✗\x1b[0m" };
        let version = check.version.as_ref().map(|v| format!(" ({})", v)).unwrap_or_default();
        let platforms = if check.required_for.is_empty() {
            " [optional]".to_string()
        } else {
            format!(" [{}]", check.required_for.join(", "))
        };

        println!("  {} {}{}{}", status, check.name, version, platforms);

        if !check.ok {
            if let Some(ref error) = check.error {
                println!("      Error: {}", error);
            }
            if let Some(ref fix) = check.fix {
                println!("      Fix: {}", fix);
            }
        }
    }

    println!("\n{}", result.summary);
}

/// Get a suggested fix for common errors (for AI consumption)
fn get_error_fix(error: &AppError) -> Option<String> {
    match error {
        AppError::ReactNativeNotFound => {
            Some("Make sure you're in a React Native project directory with package.json containing react-native dependency".to_string())
        },
        AppError::CommandFailed(cmd) => {
            if cmd.contains("pod install") {
                Some("Try: rn-run --clean-pods && rn-run --pod-install".to_string())
            } else if cmd.contains("node_modules") {
                Some("Try: rn-run --clean-modules && npm install".to_string())
            } else {
                None
            }
        },
        _ => None,
    }
}
