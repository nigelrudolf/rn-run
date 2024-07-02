use crate::args::Args;
use std::path::{PathBuf};
use crate::utils::{
    kill_process, 
    quit_simulator, 
    close_terminal_windows, 
    clean_install, 
    watch_directory,
    launch_packager,
    launch_sim,
};

pub fn run_ios(args: &Args, current_dir: &str, package_json_path: &PathBuf) {

    kill_process();
    quit_simulator();
    close_terminal_windows();

    if args.clean_install {
        clean_install(package_json_path);
    }

    watch_directory(&current_dir);

    launch_packager();

    launch_sim(args);
}