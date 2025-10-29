use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Json(serde_json::Error),
    CurrentDir,
    ReactNativeNotFound,
    CommandFailed(String),
    InvalidVersion(String),
    ProcessError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::Json(err) => write!(f, "JSON error: {}", err),
            AppError::CurrentDir => write!(f, "Failed to get current directory"),
            AppError::ReactNativeNotFound => write!(f, "package.json not found or React Native dependency missing. Make sure you're in a React Native project directory."),
            AppError::CommandFailed(cmd) => write!(f, "Command failed: {}", cmd),
            AppError::InvalidVersion(version) => write!(f, "Invalid version format: {}", version),
            AppError::ProcessError(msg) => write!(f, "Process error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            AppError::Json(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Json(err)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;