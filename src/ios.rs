use crate::args::Args;
use crate::error::Result;
use crate::utils::{
    clean_install, close_terminal_windows, deep_clean, is_version_greater_or_equal,
    kill_process_logged, launch_packager, launch_sim, quit_simulator, watch_directory_logged,
    LogWriter,
};

pub fn run_ios(args: &Args, _current_dir: &str, react_native_version: &str) -> Result<()> {
    // Create log writer at the start to capture all output
    let log = LogWriter::new("ios")?;

    kill_process_logged(Some(&log))?;
    quit_simulator()?;
    close_terminal_windows()?;

    if args.upgrade {
        deep_clean("ios")?;
    }

    if args.clean_install || args.upgrade {
        clean_install(&react_native_version, "ios")?;
    }

    // Get current directory for watchman
    let current_dir = std::env::current_dir()
        .map_err(|_| crate::error::AppError::CurrentDir)?
        .to_string_lossy()
        .to_string();

    watch_directory_logged(&current_dir, Some(&log))?;

    if is_version_greater_or_equal(react_native_version, "0.74") {
        log.log("packager will be launched via npx");
    } else {
        launch_packager()?;
    }

    let _log_path = launch_sim(&react_native_version, args, &log)?;

    Ok(())
}