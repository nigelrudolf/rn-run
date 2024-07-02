use std::path::{PathBuf};

use crate::args::Args;
use crate::utils::{
    kill_process, 
    close_terminal_windows, 
    clean_install, 
    watch_directory,
    launch_packager,
    launch_sim
};

pub fn run_android(args: &Args, current_dir: &str, package_json_path: &PathBuf) {

    kill_process();
    close_terminal_windows();

    if args.clean_install {
        clean_install(&package_json_path);
    }

    watch_directory(&current_dir);

    launch_packager();

    launch_sim(args);
}