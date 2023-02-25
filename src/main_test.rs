#[cfg(test)]
pub mod tests {
    use crate::utils::{get_current_directory, kill_process};

    #[test]
    fn test_get_current_directory() {
        let current_dir = get_current_directory().unwrap();
        assert!(current_dir.contains("/")); // Ensure that the current directory is a valid path
    }
    
    #[test]
    fn test_kill_process() {
        // start a dummy process on port 8081
        std::process::Command::new("sh")
            .arg("-c")
            .arg("sleep 10 > /dev/null 2>&1 & echo $!")
            .output()
            .unwrap();

        // kill the process
        kill_process();

        // check if the process is killed
        let output = std::process::Command::new("lsof")
            .arg("-i")
            .arg(":8081")
            .output()
            .unwrap();

        assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    }
}