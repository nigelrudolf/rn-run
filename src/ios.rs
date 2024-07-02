use crate::args::Args;
use crate::utils::{
    kill_process, 
    quit_simulator, 
    close_terminal_windows, 
    clean_install, 
    watch_directory,
    launch_packager,
    launch_sim,
};

pub fn run_ios(args: &Args, current_dir: &str, react_native_version: &str) {

    kill_process();
    quit_simulator();
    close_terminal_windows();

    if args.clean_install {
        clean_install(&react_native_version);
    }

    watch_directory(&current_dir);

    launch_packager();

    launch_sim(&react_native_version, args);
}