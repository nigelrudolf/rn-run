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

    /// Run iOS app on simulator.
    /// Kills Metro, quits simulator, optionally cleans, then builds and launches.
    /// Exit codes: 0 = success, 1 = error (check stderr or --json output)
    #[arg(short, long)]
    pub ios: bool,

    /// Run Android app on emulator.
    /// Kills Metro, optionally cleans, then builds and launches.
    /// Exit codes: 0 = success, 1 = error (check stderr or --json output)
    #[arg(short, long)]
    pub android: bool,

    /// Specify iOS simulator name (default: "iPhone 15").
    /// Use --list-simulators to see available options.
    #[arg(short, long)]
    pub simulator: Option<String>,

    // ═══════════════════════════════════════════════════════════════════════════
    // CLEANUP OPTIONS (use with -i or -a)
    // ═══════════════════════════════════════════════════════════════════════════

    /// Clean install before running.
    /// Deletes node_modules, runs npm/yarn install, runs pod install (iOS).
    /// Use when: dependency issues, node_modules corruption, new team member setup.
    #[arg(short, long)]
    pub clean_install: bool,

    /// Aggressive cleanup for React Native version upgrades.
    /// Removes: node_modules, package-lock.json, ios/Pods, ios/build,
    /// ios/Podfile.lock (iOS) or android/build, android/app/build,
    /// android/.gradle (Android). Then reinstalls everything.
    /// Use when: upgrading RN version, major dependency changes, persistent build errors.
    #[arg(short, long)]
    pub upgrade: bool,

    // ═══════════════════════════════════════════════════════════════════════════
    // OUTPUT FORMAT
    // ═══════════════════════════════════════════════════════════════════════════

    /// Output results in JSON format for programmatic parsing.
    /// All commands support this flag. Errors include structured diagnostics.
    /// Recommended for AI/automation usage.
    #[arg(long)]
    pub json: bool,

    // ═══════════════════════════════════════════════════════════════════════════
    // DIAGNOSTICS (standalone commands)
    // ═══════════════════════════════════════════════════════════════════════════

    /// Check development environment setup.
    /// Verifies: Xcode, Android SDK, node, npm/yarn, CocoaPods, watchman, Ruby.
    /// Returns detailed status of each tool with version info.
    /// Use when: build fails with "command not found" or environment errors.
    #[arg(long)]
    pub check_env: bool,

    /// Show detected React Native version from package.json.
    /// Returns: version string, package manager recommendation (npm vs yarn).
    /// Use when: need to verify RN version before running commands.
    #[arg(long)]
    pub rn_version: bool,

    /// List available iOS simulators.
    /// Returns: array of simulator names and their states (booted/shutdown).
    /// Use when: need to find valid --simulator value.
    #[arg(long)]
    pub list_simulators: bool,

    /// List available Android emulators.
    /// Returns: array of emulator names.
    /// Use when: need to verify Android emulator availability.
    #[arg(long)]
    pub list_emulators: bool,

    // ═══════════════════════════════════════════════════════════════════════════
    // PROCESS MANAGEMENT (standalone commands)
    // ═══════════════════════════════════════════════════════════════════════════

    /// Kill Metro bundler process on port 8081.
    /// Use when: Metro is stuck, port already in use, need fresh Metro start.
    /// Safe to run even if no Metro is running.
    #[arg(long)]
    pub kill_metro: bool,

    /// Quit iOS Simulator application.
    /// Use when: simulator is stuck, need to reset simulator state.
    #[arg(long)]
    pub quit_simulator: bool,

    /// Take screenshot of running iOS simulator or Android emulator.
    /// By default captures iOS simulator. Use with -a for Android.
    /// Saves to current directory with timestamp, or use --output to specify path.
    #[arg(long)]
    pub screenshot: bool,

    /// Output path for screenshot (used with --screenshot).
    /// If not specified, saves to current directory with timestamp.
    #[arg(long)]
    pub output: Option<String>,

    // ═══════════════════════════════════════════════════════════════════════════
    // TARGETED CLEANUP (standalone commands)
    // ═══════════════════════════════════════════════════════════════════════════

    /// Delete node_modules directory only.
    /// Does NOT reinstall. Run npm/yarn install manually after.
    /// Use when: need to clear node_modules without full clean install.
    #[arg(long)]
    pub clean_modules: bool,

    /// Clean iOS Pods: removes ios/Pods, ios/Podfile.lock, ios/build.
    /// Does NOT reinstall. Run --pod-install manually after.
    /// Use when: pod-related build errors, native dependency issues.
    #[arg(long)]
    pub clean_pods: bool,

    /// Clean Android Gradle: removes android/build, android/app/build, android/.gradle.
    /// Use when: gradle sync failures, Android build cache corruption.
    #[arg(long)]
    pub clean_gradle: bool,

    /// Clear Metro bundler cache (temporary files in $TMPDIR).
    /// Use when: "Unable to resolve module" errors, stale bundle, Metro cache corruption.
    #[arg(long)]
    pub clean_metro: bool,

    /// Delete all iOS simulators.
    /// Use when: simulators are corrupted, need fresh simulator setup, reclaim disk space.
    /// WARNING: This deletes ALL simulators. You'll need to recreate them in Xcode.
    #[arg(long)]
    pub delete_simulators: bool,

    // ═══════════════════════════════════════════════════════════════════════════
    // BUILD STEPS (standalone commands)
    // ═══════════════════════════════════════════════════════════════════════════

    /// Run pod install in ios/ directory.
    /// Use when: added new native iOS dependency, after cleaning pods.
    #[arg(long)]
    pub pod_install: bool,
}