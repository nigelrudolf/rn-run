use crate::args::Args;
use crate::error::Result;
use crate::utils::{
    clean_install, close_terminal_windows, deep_clean, is_version_greater_or_equal, kill_process, launch_packager, launch_sim, quit_simulator, watch_directory
};

pub fn run_ios(args: &Args, current_dir: &str, react_native_version: &str) -> Result<()> {

    kill_process()?;
    quit_simulator()?;
    close_terminal_windows()?;

    if args.upgrade {
        deep_clean("ios")?;
    }

    if args.clean_install || args.upgrade {
        clean_install(&react_native_version, "ios")?;
    }

    watch_directory(&current_dir)?;

    if is_version_greater_or_equal(react_native_version, "0.74") {
        println!("packager will be launched via npx");
    } else {
        launch_packager()?;
    }

    launch_sim(&react_native_version, args)?;
    
    Ok(())
}