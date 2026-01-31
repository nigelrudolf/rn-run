use clap::Parser;

/// Run react native app on ios or android.
///
/// For AI/automation usage, use --json flag for structured output.
/// See CLAUDE.md for detailed usage patterns and error handling guides.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // ═══════════════════════════════════════════════════════════════════════════
    // PLATFORM SELECTION
    // ═══════════════════════════════════════════════════════════════════════════

    /// Run iOS app on simulator
    #[arg(short, long, help_heading = "Platform")]
    pub ios: bool,

    /// Run Android app on emulator
    #[arg(short, long, help_heading = "Platform")]
    pub android: bool,

    /// iOS simulator name (default: "iPhone 15")
    #[arg(short, long, help_heading = "Platform")]
    pub simulator: Option<String>,

    // ═══════════════════════════════════════════════════════════════════════════
    // BUILD OPTIONS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Clean install: delete node_modules, reinstall, pod install
    #[arg(short, long, help_heading = "Build Options")]
    pub clean_install: bool,

    /// Deep clean for RN upgrades (removes all caches and locks)
    #[arg(short, long, help_heading = "Build Options")]
    pub upgrade: bool,

    // ═══════════════════════════════════════════════════════════════════════════
    // OUTPUT FORMAT
    // ═══════════════════════════════════════════════════════════════════════════

    /// Output in JSON format (for AI/automation)
    #[arg(long, help_heading = "Output")]
    pub json: bool,

    // ═══════════════════════════════════════════════════════════════════════════
    // DIAGNOSTICS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Check development environment setup
    #[arg(long, help_heading = "Diagnostics")]
    pub check_env: bool,

    /// Show React Native version from package.json
    #[arg(long, help_heading = "Diagnostics")]
    pub rn_version: bool,

    /// List available iOS simulators
    #[arg(long, help_heading = "Diagnostics")]
    pub list_simulators: bool,

    /// List available Android emulators
    #[arg(long, help_heading = "Diagnostics")]
    pub list_emulators: bool,

    // ═══════════════════════════════════════════════════════════════════════════
    // PROCESS MANAGEMENT
    // ═══════════════════════════════════════════════════════════════════════════

    /// Kill Metro bundler on port 8081
    #[arg(long, help_heading = "Process Management")]
    pub kill_metro: bool,

    /// Quit iOS Simulator app
    #[arg(long, help_heading = "Process Management")]
    pub quit_simulator: bool,

    /// Take screenshot of simulator/emulator
    #[arg(long, help_heading = "Process Management")]
    pub screenshot: bool,

    /// Output path for screenshot
    #[arg(long, help_heading = "Process Management")]
    pub output: Option<String>,

    /// Update rn-run to latest version
    #[arg(long, visible_alias = "self-update", help_heading = "Process Management")]
    pub update: bool,

    /// List recent build logs
    #[arg(long, help_heading = "Process Management")]
    pub logs: bool,

    /// Show most recent build log
    #[arg(long, help_heading = "Process Management")]
    pub show_log: bool,

    // ═══════════════════════════════════════════════════════════════════════════
    // CLEANUP
    // ═══════════════════════════════════════════════════════════════════════════

    /// Delete node_modules only
    #[arg(long, help_heading = "Cleanup")]
    pub clean_modules: bool,

    /// Clean iOS Pods, Podfile.lock, build
    #[arg(long, help_heading = "Cleanup")]
    pub clean_pods: bool,

    /// Clean Android Gradle caches
    #[arg(long, help_heading = "Cleanup")]
    pub clean_gradle: bool,

    /// Clear Metro bundler cache
    #[arg(long, help_heading = "Cleanup")]
    pub clean_metro: bool,

    /// Delete ALL iOS simulators (use with caution)
    #[arg(long, help_heading = "Cleanup")]
    pub delete_simulators: bool,

    /// Run pod install in ios/ directory
    #[arg(long, help_heading = "Cleanup")]
    pub pod_install: bool,
}
