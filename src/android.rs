use crate::args::Args;
use crate::error::Result;
use crate::utils::{
    clean_install, close_terminal_windows, is_version_greater_or_equal, kill_process, launch_packager, launch_sim, watch_directory
};

pub fn run_android(args: &Args, current_dir: &str, react_native_version: &str) -> Result<()> {

    kill_process()?;
    close_terminal_windows()?;

    if args.clean_install {
        clean_install(&react_native_version)?;
    }

    watch_directory(&current_dir)?;

    if is_version_greater_or_equal(react_native_version, "0.74")  {
        println!("packager will be launched via npx");
    } else {
        launch_packager()?;
    }

    launch_sim(&react_native_version, args)?;
    
    Ok(())
}