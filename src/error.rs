use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ZrkError {
    Io(std::io::Error),
    UnknownAgent(String),
    #[allow(dead_code)]
    ContentNotFound(String),
    #[allow(dead_code)]
    PermissionDenied(PathBuf),
    /// User-facing error from `zrk prep` with context and hints.
    Prep(String),
}

impl fmt::Display for ZrkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZrkError::Io(e) => write!(f, "I/O error: {}", e),
            ZrkError::UnknownAgent(name) => {
                write!(
                    f,
                    "Unknown agent: '{}'\n  → Supported: kiro, claude-code, cursor, windsurf",
                    name
                )
            }
            ZrkError::ContentNotFound(name) => {
                write!(f, "Content not found: '{}'", name)
            }
            ZrkError::PermissionDenied(path) => {
                write!(
                    f,
                    "Permission denied: {}\n  → Try running with appropriate permissions, or use --cwd to target a different directory.",
                    path.display()
                )
            }
            ZrkError::Prep(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ZrkError {}

impl From<std::io::Error> for ZrkError {
    fn from(e: std::io::Error) -> Self {
        ZrkError::Io(e)
    }
}
