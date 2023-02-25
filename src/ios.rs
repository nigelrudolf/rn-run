use crate::args::Args;
use crate::utils::{
    get_current_directory, 
    kill_process, 
    quit_simulator, 
    close_terminal_windows, 
    clean_install, 
    watch_directory, 
    launch_sim};

pub fn run_ios(args: &Args) {
    let watch_dir = get_current_directory();

    kill_process();
    quit_simulator();
    close_terminal_windows();

    if args.clean_install {
        clean_install();
    }

    watch_directory(&watch_dir);

    launch_sim(args);
}